/// 快速构建 Res
#[macro_export]
macro_rules! res {
    ($code:expr, $info:expr) => {
        $crate::resp::Res::new($code, $info.into(), ())
    };
    ($data:expr => $code:expr, $info:expr) => {
        $crate::resp::Res::new($code, $info.into(), $data)
    };
}

/// 快速构建Res 并支持格式化 info
#[macro_export]
macro_rules! resf {
    ($code:expr, $($msg:tt)+) => {
        $crate::resp::Res::new($code, format!($($msg)+).into(), ())
    };
    ($data:expr => $code:expr, $($msg:tt)+) => {
        $crate::resp::Res::new($code, format!($($msg)+).into(), $data)
    };
}

/// 快速响应Err Res
#[macro_export]
macro_rules! reject {
    ($($t:tt)*) => {
        Err($crate::res!($($t)*))
    };
}

/// 快速响应Err Res 并支持格式化 info
#[macro_export]
macro_rules! rejectf {
    ($($t:tt)*) => {
        Err($crate::res!($($t)*))
    };
}

/// 快速响应Ok Res
#[macro_export]
macro_rules! resolve {
    ($($t:tt)*) => {
        Ok($crate::resf!($($t)*))
    };
}

/// 快速响应Ok Res 并支持格式化 info
#[macro_export]
macro_rules! resolvef {
    ($($t:tt)*) => {
        Ok($crate::resf!($($t)*))
    };
}
