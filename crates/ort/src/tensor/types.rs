use std::fmt::{self, Debug};
#[cfg(feature = "ndarray")]
use std::{ptr, result, string};

#[cfg(feature = "ndarray")]
use super::{ortsys, Error, Result};

/// Enum mapping ONNX Runtime's supported tensor data types.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TensorElementType {
	/// 32-bit floating point number, equivalent to Rust's `f32`.
	Float32,
	/// Unsigned 8-bit integer, equivalent to Rust's `u8`.
	Uint8,
	/// Signed 8-bit integer, equivalent to Rust's `i8`.
	Int8,
	/// Unsigned 16-bit integer, equivalent to Rust's `u16`.
	Uint16,
	/// Signed 16-bit integer, equivalent to Rust's `i16`.
	Int16,
	/// Signed 32-bit integer, equivalent to Rust's `i32`.
	Int32,
	/// Signed 64-bit integer, equivalent to Rust's `i64`.
	Int64,
	/// String, equivalent to Rust's `String`.
	String,
	/// Boolean, equivalent to Rust's `bool`.
	Bool,
	/// 16-bit floating point number, equivalent to [`half::f16`] (requires the `half` feature).
	#[cfg(feature = "half")]
	#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
	Float16,
	/// 64-bit floating point number, equivalent to Rust's `f64`. Also known as `double`.
	Float64,
	/// Unsigned 32-bit integer, equivalent to Rust's `u32`.
	Uint32,
	/// Unsigned 64-bit integer, equivalent to Rust's `u64`.
	Uint64,
	// /// Complex 64-bit floating point number, equivalent to Rust's `num_complex::Complex<f64>`.
	// Complex64,
	// TODO: `num_complex` crate doesn't support i128 provided by the `decimal` crate.
	// /// Complex 128-bit floating point number, equivalent to Rust's `num_complex::Complex<f128>`.
	// Complex128,
	/// Brain 16-bit floating point number, equivalent to [`half::bf16`] (requires the `half` feature).
	#[cfg(feature = "half")]
	#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
	Bfloat16
}

impl From<TensorElementType> for ort_sys::ONNXTensorElementDataType {
	fn from(val: TensorElementType) -> Self {
		match val {
			TensorElementType::Float32 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT,
			TensorElementType::Uint8 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT8,
			TensorElementType::Int8 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT8,
			TensorElementType::Uint16 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT16,
			TensorElementType::Int16 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT16,
			TensorElementType::Int32 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT32,
			TensorElementType::Int64 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT64,
			TensorElementType::String => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING,
			TensorElementType::Bool => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_BOOL,
			#[cfg(feature = "half")]
			TensorElementType::Float16 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT16,
			TensorElementType::Float64 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_DOUBLE,
			TensorElementType::Uint32 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT32,
			TensorElementType::Uint64 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT64,
			// TensorElementDataType::Complex64 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_COMPLEX64,
			// TensorElementDataType::Complex128 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_COMPLEX128,
			#[cfg(feature = "half")]
			TensorElementType::Bfloat16 => ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_BFLOAT16
		}
	}
}
impl From<ort_sys::ONNXTensorElementDataType> for TensorElementType {
	fn from(val: ort_sys::ONNXTensorElementDataType) -> Self {
		match val {
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT => TensorElementType::Float32,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT8 => TensorElementType::Uint8,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT8 => TensorElementType::Int8,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT16 => TensorElementType::Uint16,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT16 => TensorElementType::Int16,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT32 => TensorElementType::Int32,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_INT64 => TensorElementType::Int64,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING => TensorElementType::String,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_BOOL => TensorElementType::Bool,
			#[cfg(feature = "half")]
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT16 => TensorElementType::Float16,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_DOUBLE => TensorElementType::Float64,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT32 => TensorElementType::Uint32,
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT64 => TensorElementType::Uint64,
			// ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_COMPLEX64 => TensorElementDataType::Complex64,
			// ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_COMPLEX128 => TensorElementDataType::Complex128,
			#[cfg(feature = "half")]
			ort_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_BFLOAT16 => TensorElementType::Bfloat16,
			_ => panic!("Invalid ONNXTensorElementDataType value")
		}
	}
}

/// Trait used to map Rust types (for example `f32`) to ONNX tensor element data types (for example `Float`).
pub trait IntoTensorElementType {
	/// Returns the ONNX tensor element data type corresponding to the given Rust type.
	fn into_tensor_element_type() -> TensorElementType;
}

macro_rules! impl_type_trait {
	($type_:ty, $variant:ident) => {
		impl IntoTensorElementType for $type_ {
			fn into_tensor_element_type() -> TensorElementType {
				TensorElementType::$variant
			}
		}
	};
}

impl_type_trait!(f32, Float32);
impl_type_trait!(u8, Uint8);
impl_type_trait!(i8, Int8);
impl_type_trait!(u16, Uint16);
impl_type_trait!(i16, Int16);
impl_type_trait!(i32, Int32);
impl_type_trait!(i64, Int64);
impl_type_trait!(bool, Bool);
#[cfg(feature = "half")]
#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
impl_type_trait!(half::f16, Float16);
impl_type_trait!(f64, Float64);
impl_type_trait!(u32, Uint32);
impl_type_trait!(u64, Uint64);
// impl_type_trait!(num_complex::Complex<f64>, Complex64);
// impl_type_trait!(num_complex::Complex<f128>, Complex128);
#[cfg(feature = "half")]
#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
impl_type_trait!(half::bf16, Bfloat16);

/// Adapter for common Rust string types to ONNX strings.
///
/// It should be easy to use both [`String`] and `&str` as [`TensorElementDataType::String`] data, but
/// we can't define an automatic implementation for anything that implements [`AsRef<str>`] as it
/// would conflict with the implementations of [`IntoTensorElementDataType`] for primitive numeric
/// types (which might implement [`AsRef<str>`] at some point in the future).
pub trait Utf8Data {
	/// Returns the contents of this value as a slice of UTF-8 bytes.
	fn as_utf8_bytes(&self) -> &[u8];
}

impl Utf8Data for String {
	fn as_utf8_bytes(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl<'a> Utf8Data for &'a str {
	fn as_utf8_bytes(&self) -> &[u8] {
		self.as_bytes()
	}
}

/// Trait used to map ONNX Runtime types to Rust types.
pub trait ExtractTensorData: Sized + fmt::Debug + Clone {
	/// The tensor element type that this type can extract from.
	fn tensor_element_type() -> TensorElementType;

	#[cfg(feature = "ndarray")]
	#[cfg_attr(docsrs, doc(cfg(feature = "ndarray")))]
	/// Extract an `ArrayView` from the ORT-owned tensor.
	fn extract_tensor_array<'t, D>(shape: D, tensor_element_len: usize, tensor_ptr: *mut ort_sys::OrtValue) -> Result<TensorData<'t, Self>>
	where
		D: ndarray::Dimension;
}

/// Marker type to specify that a type has the same representation in Rust as in C (which is true for every type except
/// strings), and thus a tensor value's data can be safely reinterpreted as a slice from a pointer of values.
pub trait ExtractTensorDataView: ExtractTensorData {}

/// Represents the possible ways tensor data can be accessed.
///
/// This should only be used internally.
#[derive(Debug)]
#[cfg(feature = "ndarray")]
#[cfg_attr(docsrs, doc(cfg(feature = "ndarray")))]
pub enum TensorData<'t, T> {
	/// Data residing in ONNX Runtime's tensor, in which case the `'t` lifetime is what makes this valid.
	/// This is used for data types whose in-memory form from ONNX Runtime is compatible with Rust's, like
	/// primitive numeric types.
	PrimitiveView {
		/// The pointer ONNX Runtime produced. Kept alive so that `array_view` is valid.
		ptr: *mut ort_sys::OrtValue,
		/// A view into `ptr`.
		array_view: ndarray::ArrayView<'t, T, ndarray::IxDyn>
	},
	/// String data is output differently by ONNX, and is of course also variable size, so it cannot
	/// use the same simple pointer representation.
	// Since `'t` outlives this struct, the 't lifetime is more than we need, but no harm done.
	Strings {
		/// Owned Strings copied out of ONNX Runtime's output.
		strings: ndarray::Array<T, ndarray::IxDyn>
	}
}

/// Implements [`ExtractTensorData`] for primitives which can use `GetTensorMutableData`.
macro_rules! impl_prim_type_from_ort_trait {
	($type_: ty, $variant: ident) => {
		impl ExtractTensorData for $type_ {
			fn tensor_element_type() -> TensorElementType {
				TensorElementType::$variant
			}

			#[cfg(feature = "ndarray")]
			fn extract_tensor_array<'t, D>(shape: D, _tensor_element_len: usize, tensor_ptr: *mut ort_sys::OrtValue) -> Result<TensorData<'t, Self>>
			where
				D: ndarray::Dimension
			{
				extract_primitive_array(shape, tensor_ptr).map(|v| TensorData::PrimitiveView { ptr: tensor_ptr, array_view: v })
			}
		}

		impl ExtractTensorDataView for $type_ {}
	};
}

/// Construct an [`ndarray::ArrayView`] for an ORT tensor.
///
/// Only to be used on types whose Rust in-memory representation matches ONNX Runtime's (e.g. primitive numeric types
/// like u32)
#[cfg(feature = "ndarray")]
fn extract_primitive_array<'t, D, T: ExtractTensorData>(shape: D, tensor: *mut ort_sys::OrtValue) -> Result<ndarray::ArrayView<'t, T, ndarray::IxDyn>>
where
	D: ndarray::Dimension
{
	// Get pointer to output tensor values
	let mut output_array_ptr: *mut T = ptr::null_mut();
	let output_array_ptr_ptr: *mut *mut T = &mut output_array_ptr;
	let output_array_ptr_ptr_void: *mut *mut std::ffi::c_void = output_array_ptr_ptr.cast();
	ortsys![unsafe GetTensorMutableData(tensor, output_array_ptr_ptr_void) -> Error::GetTensorMutableData; nonNull(output_array_ptr)];

	let array_view = unsafe { ndarray::ArrayView::from_shape_ptr(shape, output_array_ptr) }.into_dyn();
	Ok(array_view)
}

#[cfg(feature = "half")]
#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
impl_prim_type_from_ort_trait!(half::f16, Float16);
#[cfg(feature = "half")]
#[cfg_attr(docsrs, doc(cfg(feature = "half")))]
impl_prim_type_from_ort_trait!(half::bf16, Bfloat16);
impl_prim_type_from_ort_trait!(f32, Float32);
impl_prim_type_from_ort_trait!(f64, Float64);
impl_prim_type_from_ort_trait!(u8, Uint8);
impl_prim_type_from_ort_trait!(u16, Uint16);
impl_prim_type_from_ort_trait!(u32, Uint32);
impl_prim_type_from_ort_trait!(u64, Uint64);
impl_prim_type_from_ort_trait!(i8, Int8);
impl_prim_type_from_ort_trait!(i16, Int16);
impl_prim_type_from_ort_trait!(i32, Int32);
impl_prim_type_from_ort_trait!(i64, Int64);
impl_prim_type_from_ort_trait!(bool, Bool);

impl ExtractTensorData for String {
	fn tensor_element_type() -> TensorElementType {
		TensorElementType::String
	}

	#[cfg(feature = "ndarray")]
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	fn extract_tensor_array<'t, D: ndarray::Dimension>(
		shape: D,
		tensor_element_len: usize,
		tensor_ptr: *mut ort_sys::OrtValue
	) -> Result<TensorData<'t, Self>> {
		// Total length of string data, not including \0 suffix
		let mut total_length = 0;
		ortsys![unsafe GetStringTensorDataLength(tensor_ptr, &mut total_length) -> Error::GetStringTensorDataLength];

		// In the JNI impl of this, tensor_element_len was included in addition to total_length,
		// but that seems contrary to the docs of GetStringTensorDataLength, and those extra bytes
		// don't seem to be written to in practice either.
		// If the string data actually did go farther, it would panic below when using the offset
		// data to get slices for each string.
		let mut string_contents = vec![0u8; total_length as _];
		// one extra slot so that the total length can go in the last one, making all per-string
		// length calculations easy
		let mut offsets = vec![0; tensor_element_len + 1];

		ortsys![unsafe GetStringTensorContent(tensor_ptr, string_contents.as_mut_ptr().cast(), total_length as _, offsets.as_mut_ptr(), tensor_element_len as _) -> Error::GetStringTensorContent];

		// final offset = overall length so that per-string length calculations work for the last string
		debug_assert_eq!(0, offsets[tensor_element_len]);
		offsets[tensor_element_len] = total_length;

		let strings = offsets
            // offsets has 1 extra offset past the end so that all windows work
            .windows(2)
            .map(|w| {
                let slice = &string_contents[w[0] as _..w[1] as _];
                String::from_utf8(slice.into())
            })
            .collect::<result::Result<Vec<String>, string::FromUtf8Error>>()
            .map_err(Error::StringFromUtf8Error)?;

		let array = ndarray::Array::from_shape_vec(shape, strings)
			.expect("Shape extracted from tensor didn't match tensor contents")
			.into_dyn();

		Ok(TensorData::Strings { strings: array })
	}
}
