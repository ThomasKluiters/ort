use std::{
	ffi::CString,
	ptr::{self, NonNull}
};

use crate::{error::status_to_result, ortsys, value::ValueView, Value};

pub trait Kernel {
	fn compute(&mut self, ctx: &mut KernelContext) -> crate::Result<()>;
}

pub(crate) struct DummyKernel;

impl Kernel for DummyKernel {
	fn compute(&mut self, _: &mut KernelContext) -> crate::Result<()> {
		unimplemented!()
	}
}

pub struct KernelAttributes(NonNull<ort_sys::OrtKernelInfo>);

impl KernelAttributes {
	pub(crate) fn new(info: *const ort_sys::OrtKernelInfo) -> Self {
		Self(NonNull::from(unsafe { &*info }))
	}

	#[allow(private_bounds)]
	pub fn get<T: GetKernelAttribute>(&self, name: impl AsRef<str>) -> Option<T> {
		let name = CString::new(name.as_ref()).ok()?;
		T::get_from(self.0.as_ptr(), name.as_ptr())
	}
}

trait GetKernelAttribute {
	fn get_from(info: *mut ort_sys::OrtKernelInfo, name: *const ort_sys::c_char) -> Option<Self>
	where
		Self: Sized;
}

impl GetKernelAttribute for f32 {
	fn get_from(info: *mut ort_sys::OrtKernelInfo, name: *const ort_sys::c_char) -> Option<Self>
	where
		Self: Sized
	{
		let mut value = Self::default();
		status_to_result(ortsys![unsafe KernelInfoGetAttribute_float(info, name, &mut value)]).ok()?;
		Some(value)
	}
}

pub struct KernelContext {
	ptr: NonNull<ort_sys::OrtKernelContext>
}

impl KernelContext {
	pub(crate) fn new(ctx: *mut ort_sys::OrtKernelContext) -> Self {
		Self {
			ptr: NonNull::from(unsafe { &mut *ctx })
		}
	}

	pub fn input(&self, idx: usize) -> Option<ValueView> {
		let mut value_ptr: *const ort_sys::OrtValue = ptr::null();
		status_to_result(ortsys![unsafe KernelContext_GetInput(self.ptr.as_ptr(), idx as ort_sys::size_t, &mut value_ptr)]).ok()?;
		Some(ValueView {
			inner: unsafe { Value::from_raw_ref_dropless(value_ptr.cast_mut()) }
		})
	}

	pub fn output(&mut self, idx: usize, shape: impl IntoIterator<Item = i64>) -> Option<Value> {
		let mut value_ptr: *mut ort_sys::OrtValue = ptr::null_mut();
		let shape = shape.into_iter().collect::<Vec<i64>>();
		status_to_result(ortsys![unsafe KernelContext_GetOutput(self.ptr.as_ptr(), idx as ort_sys::size_t, shape.as_ptr(), shape.len() as _, &mut value_ptr)])
			.ok()?;
		assert!(!value_ptr.is_null());
		Some(unsafe { Value::from_raw_ref_dropless(value_ptr) })
	}
}
