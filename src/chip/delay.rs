//! WCH SysTick 计数延时的**纯算术**(host 可测)——被 `ch583`/`ch572` 的
//! `Porting::delay_us` 使用。
//!
//! 背景:SysTick 计数器按 `CMP` 自动重装,周期 `PERIOD = TICKS + 1` 拍。
//! 要等 N 拍,必须按"到下次重装还剩多少"**分块**:直接比较 `now - start`
//! 在跨重装时会算错(计数器回绕 → 朴素差值瞬间变大,延时提前结束;
//! 用有符号比较则可能多等一整周期)。这里把分块与差值都抽成纯函数,
//! 由 host 单测钉住边界(重装点前后、起点为 0、块长正好等于剩余空间)。

/// 从 `start` 到下次重装还剩多少拍(取值 `1..=period`;`start % period == 0` 时为 `period`)
#[inline]
pub(crate) fn room_to_reload(start: u64, period: u64) -> u64 {
    period - (start % period)
}

/// 从 `start` 到 `now` 已经过去多少拍(跨重装按模计算,结果 `0..period`)
#[inline]
pub(crate) fn elapsed(start: u64, now: u64, period: u64) -> u64 {
    let s = start % period;
    let n = now % period;
    (n + period - s) % period
}

/// 本次迭代能等多少拍:`remaining` 与"到下次重装还剩多少"取小,
/// **再钳到 `period - 1`**。
///
/// 钳这一下是必须的:`elapsed()` 用模运算表示,取值只能到 `period - 1`;
/// 若 `chunk == period`(起点恰好落在重装点上时 `room_to_reload` 就是整周期),
/// "等到 chunk 拍"这个条件永远不成立 → **死循环**(本文件单测
/// `chunk_never_equals_full_period` 就是钉这个的)。
/// 分块只影响迭代次数,总拍数仍精确。
#[inline]
pub(crate) fn next_chunk(remaining: u64, start: u64, period: u64) -> u64 {
    remaining.min(room_to_reload(start, period)).min(period - 1)
}

#[cfg(test)]
mod tests {
    use super::{elapsed, next_chunk, room_to_reload};

    const PERIOD: u64 = 10;

    #[test]
    fn room_to_reload_boundaries() {
        assert_eq!(room_to_reload(0, PERIOD), PERIOD); // 刚好在重装点:整周期可用
        assert_eq!(room_to_reload(1, PERIOD), PERIOD - 1);
        assert_eq!(room_to_reload(PERIOD - 1, PERIOD), 1); // 下一拍就重装
        assert_eq!(room_to_reload(PERIOD, PERIOD), PERIOD); // 与 0 等价
        assert_eq!(room_to_reload(2 * PERIOD + 3, PERIOD), PERIOD - 3); // 多圈后取模
    }

    #[test]
    fn elapsed_within_period() {
        assert_eq!(elapsed(0, 0, PERIOD), 0);
        assert_eq!(elapsed(3, 7, PERIOD), 4);
        assert_eq!(elapsed(3, 3, PERIOD), 0);
    }

    #[test]
    fn elapsed_across_reload() {
        // 起点 8,计数器走到 2(已回绕一圈):过去 4 拍,不是 2-8 那种负数
        assert_eq!(elapsed(8, 2, PERIOD), 4);
        // 起点 9,走到 0:过去 1 拍
        assert_eq!(elapsed(9, 0, PERIOD), 1);
        // 起点 0,走到任意值就是该值
        assert_eq!(elapsed(0, 5, PERIOD), 5);
    }

    #[test]
    fn chunk_never_equals_full_period() {
        // 回归:起点落在重装点上(room = 整周期)时必须钳到 period-1,
        // 否则 elapsed()(模运算,最大 period-1)永远够不到 chunk → 死循环
        assert_eq!(next_chunk(1000, 0, PERIOD), PERIOD - 1);
        for start in 0..PERIOD {
            assert!(next_chunk(1000, start, PERIOD) < PERIOD);
        }
    }

    /// 用纯算术复演分块等待:返回实际等了多少拍(计数器按拍推进、回绕)
    fn simulate(mut cnt: u64, mut remaining: u64) -> u64 {
        let mut waited = 0u64;
        let mut guard = 0usize;
        while remaining > 0 {
            guard += 1;
            assert!(guard < 10_000, "分块循环没收敛(死循环回归)");
            let start = cnt;
            let chunk = next_chunk(remaining, start, PERIOD);
            while elapsed(start, cnt, PERIOD) < chunk {
                cnt = (cnt + 1) % PERIOD;
                waited += 1;
                guard += 1;
                assert!(guard < 10_000, "等待循环没收敛(死循环回归)");
            }
            remaining -= chunk;
        }
        waited
    }

    #[test]
    fn chunking_waits_exact_total() {
        // 已覆盖:起点落在重装点(0)、周期中段(7)、末尾(9),以及跨多周期的大延时
        assert_eq!(simulate(0, 15), 15);
        assert_eq!(simulate(7, 15), 15);
        assert_eq!(simulate(9, 1), 1);
        assert_eq!(simulate(3, 0), 0);
        assert_eq!(simulate(5, 137), 137);
    }
}
