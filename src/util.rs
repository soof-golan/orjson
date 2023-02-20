// SPDX-License-Identifier: (Apache-2.0 OR MIT)

macro_rules! is_type {
    ($obj_ptr:expr, $type_ptr:expr) => {
        unsafe { $obj_ptr == $type_ptr }
    };
}

macro_rules! ob_type {
    ($obj:expr) => {
        unsafe { (*$obj).ob_type }
    };
}

macro_rules! err {
    ($msg:expr) => {
        return Err(serde::ser::Error::custom($msg))
    };
}

#[cfg(feature = "intrinsics")]
macro_rules! unlikely {
    ($exp:expr) => {
        std::intrinsics::unlikely($exp)
    };
}

#[cfg(not(feature = "intrinsics"))]
macro_rules! unlikely {
    ($exp:expr) => {
        $exp
    };
}

macro_rules! nonnull {
    ($exp:expr) => {
        unsafe { std::ptr::NonNull::new_unchecked($exp) }
    };
}

macro_rules! str_from_slice {
    ($ptr:expr, $size:expr) => {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts($ptr, $size as usize)) }
    };
}

macro_rules! py_decref_without_destroy {
    ($op:expr) => {
        unsafe {
            (*$op).ob_refcnt -= 1;
        }
    };
}

macro_rules! ffi {
    ($fn:ident()) => {
        unsafe { pyo3_ffi::$fn() }
    };

    ($fn:ident($obj1:expr)) => {
        unsafe { pyo3_ffi::$fn($obj1) }
    };

    ($fn:ident($obj1:expr, $obj2:expr)) => {
        unsafe { pyo3_ffi::$fn($obj1, $obj2) }
    };

    ($fn:ident($obj1:expr, $obj2:expr, $obj3:expr)) => {
        unsafe { pyo3_ffi::$fn($obj1, $obj2, $obj3) }
    };

    ($fn:ident($obj1:expr, $obj2:expr, $obj3:expr, $obj4:expr)) => {
        unsafe { pyo3_ffi::$fn($obj1, $obj2, $obj3, $obj4) }
    };
}

#[cfg(Py_3_9)]
macro_rules! call_method {
    ($obj1:expr, $obj2:expr) => {
        unsafe { pyo3_ffi::PyObject_CallMethodNoArgs($obj1, $obj2) }
    };
    ($obj1:expr, $obj2:expr, $obj3:expr) => {
        unsafe { pyo3_ffi::PyObject_CallMethodOneArg($obj1, $obj2, $obj3) }
    };
}

#[cfg(not(Py_3_9))]
macro_rules! call_method {
    ($obj1:expr, $obj2:expr) => {
        unsafe {
            pyo3_ffi::PyObject_CallMethodObjArgs(
                $obj1,
                $obj2,
                std::ptr::null_mut() as *mut pyo3_ffi::PyObject,
            )
        }
    };
    ($obj1:expr, $obj2:expr, $obj3:expr) => {
        unsafe {
            pyo3_ffi::PyObject_CallMethodObjArgs(
                $obj1,
                $obj2,
                $obj3,
                std::ptr::null_mut() as *mut pyo3_ffi::PyObject,
            )
        }
    };
}

#[inline(always)]
pub fn iter_next(iter: *mut pyo3_ffi::PyObject) -> Option<*mut pyo3_ffi::PyObject> {
    let elem = ffi!(PyIter_Next(iter));
    if unlikely!(elem.is_null()) {
        None
    } else {
        Some(elem)
    }
}

#[inline(always)]
pub fn get_iter(obj: *mut pyo3_ffi::PyObject) -> Option<*mut pyo3_ffi::PyObject> {
    let iter = ffi!(PyObject_GetIter(obj));
    if unlikely!(iter.is_null()) {
        None
    } else {
        Some(iter)
    }
}
