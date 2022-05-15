//! 卡尔曼滤波
//! 代码来自互联网

use alloc::vec::Vec;
use nalgebra::{DMatrix, DVector};

// #[derive(Debug, Default)]
// struct KalmanScalar {
//     x: f32,    // 系统的状态量
//     a: f32,    // x(n)=A*x(n-1)+u(n),u(n)~N(0,q)
//     h: f32,    // z(n)=H*x(n)+w(n),w(n)~N(0,r)
//     q: f32,    // 预测过程噪声协方差
//     r: f32,    // 测量过程噪声协方差
//     p: f32,    // 估计误差协方差
//     gain: f32, //卡尔曼增益
// }

// impl KalmanScalar {
//     /**
//      *@x：待测量的初始值
//      *@p：后验状态估计值误差的方差的初始值
//      */
//     fn new(x: f32, p: f32, predict_q: f32, new_measured_q: f32) -> Self {
//         Self {
//             x: x,              //待测量的初始值，如有中值一般设成中值
//             p: p,              //后验状态估计值误差的方差的初始值（不要为0问题不大）
//             q: predict_q,      //预测（过程）噪声方差 影响收敛速率，可以根据实际需求给出
//             r: new_measured_q, //测量（观测）噪声方差 可以通过实验手段获得
//             a: 1.0,
//             h: 1.0,
//             gain: 0.0,
//         }
//     }
//     /**
//      *@new_measured；测量值
//      *返回滤波后的值
//      */
//     fn filter(&mut self, new_measured: f32) -> f32 {
//         /* Predict */
//         self.x = self.a * self.x; //%x的先验估计由上一个时间点的后验估计值和输入信息给出
//         self.p = self.a * self.a * self.p + self.q; /*计算先验均方差 p(n|n-1)=A^2*p(n-1|n-1)+q */
//         /* Correct */
//         self.gain = self.p * self.h / (self.p * self.h * self.h + self.r);
//         self.x = self.x + self.gain * (new_measured - self.h * self.x); //利用残余的信息改善对x(t)的估计，给出后验估计，这个值也就是输出
//         self.p = (1.0 - self.gain * self.h) * self.p; //%计算后验均方差
//         self.x
//     }
// }

//标量卡尔曼滤波器
#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct ScalarKalmanFilter {
    A: f32,
    C: f32,
    A2: f32,
    C2: f32,
    Q: f32,
    R: f32,
    K: f32,
    P: f32,
    x: f32,
}

impl ScalarKalmanFilter {
    pub fn new(a: f32, c: f32, q: f32, r: f32) -> Self {
        Self {
            A: a,
            C: c,
            Q: q,
            R: r,
            A2: a * a,
            C2: c * c,
            P: q,
            K: 1.0,
            x: 0.0,
        }
    }

    pub fn filter(&mut self, y: f32) -> f32 {
        //状态预测
        self.x = self.A * self.x;
        // 误差协方差预测
        self.P = self.A2 * self.P + self.Q;
        // 计算卡尔曼滤波增益
        self.K = self.P * self.C / (self.C2 * self.P + self.R);
        // 状态估计校正
        self.x = self.x + self.K * (y - self.C * self.x);
        // 误差协方差估计校正
        self.P = (1.0 - self.K * self.C) * self.P;
        // 输出滤波后的y
        self.C * self.x
    }
}

/// # Fields
/// * `q`: process noise covariance
/// * `r`: measurement noise covariance
/// * `h`: observation matrix
/// * `f`: state transition matrix
/// * `x0`: initial guess for state mean at time 1
/// * `p0`: initial guess for state covariance at time 1
#[derive(Clone, Debug)]
pub struct KalmanState {
    pub x: DVector<f32>, // State vector
    pub p: DMatrix<f32>, // State covariance
}

//线性卡尔曼过滤器
#[derive(Debug)]
pub struct LinearKalmanFilter {
    pub q: DMatrix<f32>,  // 过程噪声协方差
    pub r: DMatrix<f32>,  // 测量噪声协方差
    pub h: DMatrix<f32>,  // 观察矩阵
    pub f: DMatrix<f32>,  // 状态转移矩阵
    pub x0: DVector<f32>, // 状态变量初始值
    pub p0: DMatrix<f32>, // 状态协方差初始值
}

impl LinearKalmanFilter {
    pub fn new(
        q: DMatrix<f32>,
        r: DMatrix<f32>,
        h: DMatrix<f32>,
        f: DMatrix<f32>,
        x0: DVector<f32>,
        p0: DMatrix<f32>,
    ) -> Self {
        Self { q, r, h, f, x0, p0 }
    }

    pub fn filter(&mut self, data: &Vec<DVector<f32>>) -> (Vec<KalmanState>, Vec<KalmanState>) {
        let t: usize = data.len();
        // Containers for predicted and filtered estimates
        let mut predicted: Vec<KalmanState> = Vec::with_capacity(t + 1);
        let mut filtered: Vec<KalmanState> = Vec::with_capacity(t);

        predicted.push(KalmanState {
            x: (self.x0).clone(),
            p: (self.p0).clone(),
        });

        for k in 0..t {
            filtered.push(update_step(self, &predicted[k], &data[k]));
            predicted.push(predict_step(self, &filtered[k]));
        }

        (filtered, predicted)
    }
    pub fn smooth(
        &self,
        filtered: &Vec<KalmanState>,
        predicted: &Vec<KalmanState>,
    ) -> Vec<KalmanState> {
        let t: usize = filtered.len();
        let mut smoothed: Vec<KalmanState> = Vec::with_capacity(t);

        // Do Kalman smoothing in reverse order
        let mut init = (filtered[t - 1]).clone();
        smoothed.push((filtered[t - 1]).clone());

        for k in 1..t {
            smoothed.push(smoothing_step(
                self,
                &init,
                &filtered[t - k - 1],
                &predicted[t - k],
            ));
            init = (&smoothed[k]).clone();
        }

        smoothed.reverse();
        smoothed
    }
}

/// Returns a predicted state variable mean and covariance. If prediction for
/// time `t` is desired, then `KalmanState` object with initial conditions
/// contains state mean and covariance at time `t-1` given information up to
/// time `t-1`.
pub fn predict_step(kalman_filter: &LinearKalmanFilter, init: &KalmanState) -> KalmanState {
    // Predict state variable and covariance
    let xp: DVector<f32> = &kalman_filter.f * &init.x;
    let pp: DMatrix<f32> =
        &kalman_filter.f * &init.p * &kalman_filter.f.transpose() + &kalman_filter.q;

    KalmanState { x: xp, p: pp }
}

/// Returns an updated state variable mean and covariance given predicted and
/// observed data. Typically, update step will be called after prediction step,
/// data of which will be consequently used as input in updating.
pub fn update_step(
    kalman_filter: &LinearKalmanFilter,
    pred: &KalmanState,
    measure: &DVector<f32>,
) -> KalmanState {
    let mut identity = kalman_filter.x0.clone();
    identity.fill_with_identity();
    // let identity = Matrix::<f32>::identity(kalman_filter.x0.size());

    // Compute Kalman gain
    let k = &pred.p
        * &kalman_filter.h.transpose()
        * (&kalman_filter.h * &pred.p * &kalman_filter.h.transpose() + &kalman_filter.r)
            .try_inverse()
            .expect("Kalman gain computation failed due to failure to invert.");

    // Update state variable and covariance
    let x = &pred.x + &k * (measure - &kalman_filter.h * &pred.x);
    let p = (identity - &k * &kalman_filter.h) * &pred.p;

    KalmanState { x: x, p: p }
}

/// Returns a tuple containing updated and predicted estimates (in that order)
/// of the state variable and its covariance. This function might be useful for
/// cases where data is incoming and being updated in real-time so that Kalman
/// filtering is run incrementally. Note that given some initial values for `x`
/// and `P`, `filter_step` makes a prediction and then runs the update step to
/// correct the prediction based on the observed data.
pub fn filter_step(
    kalman_filter: &LinearKalmanFilter,
    init: &KalmanState,
    measure: &DVector<f32>,
) -> (KalmanState, KalmanState) {
    let pred = predict_step(kalman_filter, init);
    let upd = update_step(kalman_filter, &pred, measure);

    (
        KalmanState { x: upd.x, p: upd.p },
        KalmanState {
            x: pred.x,
            p: pred.p,
        },
    )
}

fn smoothing_step(
    kalman_filter: &LinearKalmanFilter,
    init: &KalmanState,
    filtered: &KalmanState,
    predicted: &KalmanState,
) -> KalmanState {
    let j = &filtered.p
        * &kalman_filter.f.transpose()
        * &predicted
            .p
            .clone()
            .try_inverse()
            .expect("Predicted state covariance matrix could not be inverted.");
    let x: DVector<f32> = &filtered.x + &j * (&init.x - &predicted.x);
    let p: DMatrix<f32> = &filtered.p + &j * (&init.p - &predicted.p) * &j.transpose();

    KalmanState { x: x, p: p }
}

///////////////////////////////////////////////

// #include "stm32f10x.h"
// #include "filter.h"
// #include <stdio.h>

// static float angle,angle_dot;
// const  float Q_angle = 0.002, Q_gyro = 0.002, R_angle = 0.5, dt = 0.01;
// static float P[2][2]-{
//                       { 1 , 0 },
//                       { 0 , 1 }
//                      };
// static float Pdot[4] = { 0 , 0 , 0 , 0};
// const  u8    C_0 = 1;
// static float q_bias , angle_err , PCt_0 , PCt_1 , E , K_0 , K_1 , t_0 , t_1;

// /* float angle_m 加速度计计算角度
//    float gyro_m  陀螺仪角速度
//    float *angle_f 融合后的角度
//    float *angle_dot_f 融合后的角速度

//    Q_angle 加速度计 过程噪声协方差(仿真整定)
//    Q_gyro  陀螺仪   过程噪声协方差(仿真整定)
//    R_angle 加速度计 测量方程协方差(对测量设备进行数据测量后利用matlab进行协方差计算)
//    P    角度协方差矩阵
//    Pdot 角速度协方差矩阵
//    dt   微分因子
//    q_bias 陀螺仪偏移量
//    C0 测量矩阵
//    K0 K1 卡尔曼增益矩阵(2 * 1)元素
//    PCt_0 PCt_1 t_0 t_1 中间计算变量
//    angle_err 过程测量误差值
//    void kalman_fliter(float angle_m, float gyro_m, float *angle_f , *angle_dot_f)
// */
// void kalman_filter(float angle_m, float gyro_m, float *angle_f, float *angle_dot_f)
// {
// 	angle += (gyro_m - q_bias) * dt;

// 	Pdot[0] = Q_angle - P[0][1] - P[1][0];
// 	Pdot[1] = -P[1][1];
// 	Pdot[2] = -P[1][1];
// 	Pdot[3] = Q_gyro;

// 	P[0][0] += Pdot[0] * dt;
// 	P[0][1] += Pdot[1] * dt;
// 	P[1][0] += Pdot[2] * dt;
// 	P[1][1] += Pdot[3] * dt;

// 	angle_err = angle_m - angle;

// 	PCt_0 = C_0 * P[0][0];
// 	PCt_1 = C_0 * P[1][0];

// 	E = R_angle + C_0 * PCt_0;

// 	K_0 = PCt_0 / E;
// 	K_1 = PCt_1 / E;

// 	t_0 = PCt_0;
// 	t_1 = C_0 * P[0][1];

// 	P[0][0] -= K_0 * t_0;
// 	P[0][1] -= K_0 * t_1;
// 	P[1][0] -= K_1 * t_0;
// 	P[1][1] -= K_1 * t_1;

// 	angle  += K_0 * angle_err;
// 	q_bias += K_1 * angle_err;
// 	angle_dot = gyro_m - q_bias;

// 	*angle_f        = angle;
// 	*angle_dot_f = angle_dot;
// }
