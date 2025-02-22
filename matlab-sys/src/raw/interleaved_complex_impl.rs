/* automatically generated by rust-bindgen 0.64.0 */
#![allow(nonstandard_style)]
#[cfg(not(target_pointer_width = "64"))]
compile_error!("The bindings are only valid for 64-bit applications. All Matlab versions after 2015b are only available in 64-bit.");
pub type FILE = _iobuf;
pub type mwSize = usize;
pub type mwIndex = usize;
pub type mwSignedIndex = isize;
#[doc = " Forward declaration for mxArray"]
pub type mxArray = mxArray_tag;
#[doc = " MEX-file entry point type"]
pub type mxFunctionPtr = ::core::option::Option<
    unsafe extern "C" fn(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *mut mxArray,
    ),
>;
#[doc = " Logical type"]
pub type mxLogical = bool;
#[doc = " Required for Unicode support in MATLAB"]
pub type mxChar = u16;
pub type mex_exit_fn = ::core::option::Option<unsafe extern "C" fn()>;
pub type MATFile = MatFile_tag;
pub type matError = ::core::ffi::c_int;
pub type Engine = engine;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mxArray_tag {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mxComplexDouble {
    pub real: f64,
    pub imag: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mxComplexSingle {
    pub real: f32,
    pub imag: f32,
}
#[repr(C)]
pub struct mxComplexInt8 {
    pub real: i8,
    pub imag: i8,
}
#[repr(C)]
pub struct mxComplexUint8 {
    pub real: u8,
    pub imag: u8,
}
#[repr(C)]
pub struct mxComplexInt16 {
    pub real: i16,
    pub imag: i16,
}
#[repr(C)]
pub struct mxComplexUint16 {
    pub real: u16,
    pub imag: u16,
}
#[repr(C)]
pub struct mxComplexInt32 {
    pub real: i32,
    pub imag: i32,
}
#[repr(C)]
pub struct mxComplexUint32 {
    pub real: u32,
    pub imag: u32,
}
#[repr(C)]
pub struct mxComplexInt64 {
    pub real: i64,
    pub imag: i64,
}
#[repr(C)]
pub struct mxComplexUint64 {
    pub real: u64,
    pub imag: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MatFile_tag {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct engine {
    _unused: [u8; 0],
}
pub const MWSIZE_MAX: u64 = 281474976710655;
pub const MWINDEX_MAX: u64 = 281474976710655;
pub const MWSINDEX_MAX: u64 = 281474976710655;
pub const MWSINDEX_MIN: i64 = -281474976710655;
pub const MWSIZE_MIN: u32 = 0;
pub const MWINDEX_MIN: u32 = 0;
pub const mxMAXNAM: u32 = 64;
pub mod mxClassID {
    #[doc = " mxArray classes."]
    pub type Type = i32;
    pub const mxUNKNOWN_CLASS: Type = 0;
    pub const mxCELL_CLASS: Type = 1;
    pub const mxSTRUCT_CLASS: Type = 2;
    pub const mxLOGICAL_CLASS: Type = 3;
    pub const mxCHAR_CLASS: Type = 4;
    pub const mxVOID_CLASS: Type = 5;
    pub const mxDOUBLE_CLASS: Type = 6;
    pub const mxSINGLE_CLASS: Type = 7;
    pub const mxINT8_CLASS: Type = 8;
    pub const mxUINT8_CLASS: Type = 9;
    pub const mxINT16_CLASS: Type = 10;
    pub const mxUINT16_CLASS: Type = 11;
    pub const mxINT32_CLASS: Type = 12;
    pub const mxUINT32_CLASS: Type = 13;
    pub const mxINT64_CLASS: Type = 14;
    pub const mxUINT64_CLASS: Type = 15;
    pub const mxFUNCTION_CLASS: Type = 16;
    pub const mxOPAQUE_CLASS: Type = 17;
    pub const mxOBJECT_CLASS: Type = 18;
    pub const mxINDEX_CLASS: Type = 15;
}
pub mod mxComplexity {
    #[doc = " Indicates whether floating-point mxArrays are real or complex."]
    pub type Type = i32;
    pub const mxREAL: Type = 0;
    pub const mxCOMPLEX: Type = 1;
}
extern "C" {
    #[link_name = "mxMalloc_800"]
    pub fn mxMalloc(n: usize) -> *mut ::core::ffi::c_void;
    #[link_name = "mxCalloc_800"]
    pub fn mxCalloc(n: usize, size: usize) -> *mut ::core::ffi::c_void;
    #[link_name = "mxFree_800"]
    pub fn mxFree(ptr: *mut ::core::ffi::c_void);
    #[link_name = "mxRealloc_800"]
    pub fn mxRealloc(ptr: *mut ::core::ffi::c_void, size: usize) -> *mut ::core::ffi::c_void;
    #[link_name = "mxGetNumberOfDimensions_800"]
    pub fn mxGetNumberOfDimensions(pa: *const mxArray) -> mwSize;
    #[link_name = "mxGetDimensions_800"]
    pub fn mxGetDimensions(pa: *const mxArray) -> *const mwSize;
    #[link_name = "mxGetM_800"]
    pub fn mxGetM(pa: *const mxArray) -> usize;
    #[link_name = "mxGetIr_800"]
    pub fn mxGetIr(pa: *const mxArray) -> *mut mwIndex;
    #[link_name = "mxGetJc_800"]
    pub fn mxGetJc(pa: *const mxArray) -> *mut mwIndex;
    #[link_name = "mxGetNzmax_800"]
    pub fn mxGetNzmax(pa: *const mxArray) -> mwSize;
    #[link_name = "mxSetNzmax_800"]
    pub fn mxSetNzmax(pa: *mut mxArray, nzmax: mwSize);
    #[link_name = "mxGetFieldNameByNumber_800"]
    pub fn mxGetFieldNameByNumber(
        pa: *const mxArray,
        n: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "mxGetFieldByNumber_800"]
    pub fn mxGetFieldByNumber(
        pa: *const mxArray,
        i: mwIndex,
        fieldnum: ::core::ffi::c_int,
    ) -> *mut mxArray;
    #[link_name = "mxGetCell_800"]
    pub fn mxGetCell(pa: *const mxArray, i: mwIndex) -> *mut mxArray;
    #[link_name = "mxGetClassID_800"]
    pub fn mxGetClassID(pa: *const mxArray) -> mxClassID::Type;
    #[link_name = "mxGetData_800"]
    pub fn mxGetData(pa: *const mxArray) -> *mut ::core::ffi::c_void;
    #[link_name = "mxSetData_800"]
    pub fn mxSetData(pa: *mut mxArray, newdata: *mut ::core::ffi::c_void);
    #[link_name = "mxIsNumeric_800"]
    pub fn mxIsNumeric(pa: *const mxArray) -> bool;
    #[link_name = "mxIsCell_800"]
    pub fn mxIsCell(pa: *const mxArray) -> bool;
    #[link_name = "mxIsLogical_800"]
    pub fn mxIsLogical(pa: *const mxArray) -> bool;
    #[link_name = "mxIsScalar_800"]
    pub fn mxIsScalar(pa: *const mxArray) -> bool;
    #[link_name = "mxIsChar_800"]
    pub fn mxIsChar(pa: *const mxArray) -> bool;
    #[link_name = "mxIsStruct_800"]
    pub fn mxIsStruct(pa: *const mxArray) -> bool;
    #[link_name = "mxIsOpaque_800"]
    pub fn mxIsOpaque(pa: *const mxArray) -> bool;
    #[link_name = "mxIsFunctionHandle_800"]
    pub fn mxIsFunctionHandle(pa: *const mxArray) -> bool;
    #[link_name = "mxIsObject_800"]
    pub fn mxIsObject(pa: *const mxArray) -> bool;
    #[link_name = "mxIsComplex_800"]
    pub fn mxIsComplex(pa: *const mxArray) -> bool;
    #[link_name = "mxIsSparse_800"]
    pub fn mxIsSparse(pa: *const mxArray) -> bool;
    #[link_name = "mxIsDouble_800"]
    pub fn mxIsDouble(pa: *const mxArray) -> bool;
    #[link_name = "mxIsSingle_800"]
    pub fn mxIsSingle(pa: *const mxArray) -> bool;
    #[link_name = "mxIsInt8_800"]
    pub fn mxIsInt8(pa: *const mxArray) -> bool;
    #[link_name = "mxIsUint8_800"]
    pub fn mxIsUint8(pa: *const mxArray) -> bool;
    #[link_name = "mxIsInt16_800"]
    pub fn mxIsInt16(pa: *const mxArray) -> bool;
    #[link_name = "mxIsUint16_800"]
    pub fn mxIsUint16(pa: *const mxArray) -> bool;
    #[link_name = "mxIsInt32_800"]
    pub fn mxIsInt32(pa: *const mxArray) -> bool;
    #[link_name = "mxIsUint32_800"]
    pub fn mxIsUint32(pa: *const mxArray) -> bool;
    #[link_name = "mxIsInt64_800"]
    pub fn mxIsInt64(pa: *const mxArray) -> bool;
    #[link_name = "mxIsUint64_800"]
    pub fn mxIsUint64(pa: *const mxArray) -> bool;
    #[link_name = "mxGetNumberOfElements_800"]
    pub fn mxGetNumberOfElements(pa: *const mxArray) -> usize;
    #[link_name = "mxGetChars_800"]
    pub fn mxGetChars(pa: *const mxArray) -> *mut mxChar;
    #[link_name = "mxGetUserBits_800"]
    pub fn mxGetUserBits(pa: *const mxArray) -> ::core::ffi::c_int;
    #[link_name = "mxSetUserBits_800"]
    pub fn mxSetUserBits(pa: *mut mxArray, value: ::core::ffi::c_int);
    #[link_name = "mxGetScalar_800"]
    pub fn mxGetScalar(pa: *const mxArray) -> f64;
    #[link_name = "mxIsFromGlobalWS_800"]
    pub fn mxIsFromGlobalWS(pa: *const mxArray) -> bool;
    #[link_name = "mxSetFromGlobalWS_800"]
    pub fn mxSetFromGlobalWS(pa: *mut mxArray, global: bool);
    #[link_name = "mxSetM_800"]
    pub fn mxSetM(pa: *mut mxArray, m: mwSize);
    #[link_name = "mxGetN_800"]
    pub fn mxGetN(pa: *const mxArray) -> usize;
    #[link_name = "mxIsEmpty_800"]
    pub fn mxIsEmpty(pa: *const mxArray) -> bool;
    #[link_name = "mxGetFieldNumber_800"]
    pub fn mxGetFieldNumber(
        pa: *const mxArray,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxSetIr_800"]
    pub fn mxSetIr(pa: *mut mxArray, newir: *mut mwIndex);
    #[link_name = "mxSetJc_800"]
    pub fn mxSetJc(pa: *mut mxArray, newjc: *mut mwIndex);
    #[link_name = "mxGetPr_800"]
    pub fn mxGetPr(pa: *const mxArray) -> *mut f64;
    #[link_name = "mxSetPr_800"]
    pub fn mxSetPr(pa: *mut mxArray, newdata: *mut f64);
    #[link_name = "mxGetElementSize_800"]
    pub fn mxGetElementSize(pa: *const mxArray) -> usize;
    #[link_name = "mxCalcSingleSubscript_800"]
    pub fn mxCalcSingleSubscript(
        pa: *const mxArray,
        nsubs: mwSize,
        subs: *const mwIndex,
    ) -> mwIndex;
    #[link_name = "mxGetNumberOfFields_800"]
    pub fn mxGetNumberOfFields(pa: *const mxArray) -> ::core::ffi::c_int;
    #[link_name = "mxSetCell_800"]
    pub fn mxSetCell(pa: *mut mxArray, i: mwIndex, value: *mut mxArray);
    #[link_name = "mxSetFieldByNumber_800"]
    pub fn mxSetFieldByNumber(
        pa: *mut mxArray,
        i: mwIndex,
        fieldnum: ::core::ffi::c_int,
        value: *mut mxArray,
    );
    #[link_name = "mxGetField_800"]
    pub fn mxGetField(
        pa: *const mxArray,
        i: mwIndex,
        fieldname: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "mxSetField_800"]
    pub fn mxSetField(
        pa: *mut mxArray,
        i: mwIndex,
        fieldname: *const ::core::ffi::c_char,
        value: *mut mxArray,
    );
    #[link_name = "mxGetProperty_800"]
    pub fn mxGetProperty(
        pa: *const mxArray,
        i: mwIndex,
        propname: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "mxSetProperty_800"]
    pub fn mxSetProperty(
        pa: *mut mxArray,
        i: mwIndex,
        propname: *const ::core::ffi::c_char,
        value: *const mxArray,
    );
    #[link_name = "mxGetClassName_800"]
    pub fn mxGetClassName(pa: *const mxArray) -> *const ::core::ffi::c_char;
    #[link_name = "mxIsClass_800"]
    pub fn mxIsClass(pa: *const mxArray, name: *const ::core::ffi::c_char) -> bool;
    #[link_name = "mxCreateNumericMatrix_800"]
    pub fn mxCreateNumericMatrix(
        m: mwSize,
        n: mwSize,
        classid: mxClassID::Type,
        flag: mxComplexity::Type,
    ) -> *mut mxArray;
    #[link_name = "mxCreateUninitNumericMatrix_800"]
    pub fn mxCreateUninitNumericMatrix(
        m: usize,
        n: usize,
        classid: mxClassID::Type,
        flag: mxComplexity::Type,
    ) -> *mut mxArray;
    #[link_name = "mxCreateUninitNumericArray_800"]
    pub fn mxCreateUninitNumericArray(
        ndim: usize,
        dims: *mut usize,
        classid: mxClassID::Type,
        flag: mxComplexity::Type,
    ) -> *mut mxArray;
    #[link_name = "mxSetN_800"]
    pub fn mxSetN(pa: *mut mxArray, n: mwSize);
    #[link_name = "mxSetDimensions_800"]
    pub fn mxSetDimensions(
        pa: *mut mxArray,
        pdims: *const mwSize,
        ndims: mwSize,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxDestroyArray_800"]
    pub fn mxDestroyArray(pa: *mut mxArray);
    #[link_name = "mxCreateNumericArray_800"]
    pub fn mxCreateNumericArray(
        ndim: mwSize,
        dims: *const mwSize,
        classid: mxClassID::Type,
        flag: mxComplexity::Type,
    ) -> *mut mxArray;
    #[link_name = "mxCreateCharArray_800"]
    pub fn mxCreateCharArray(ndim: mwSize, dims: *const mwSize) -> *mut mxArray;
    #[link_name = "mxCreateDoubleMatrix_800"]
    pub fn mxCreateDoubleMatrix(m: mwSize, n: mwSize, flag: mxComplexity::Type) -> *mut mxArray;
    #[link_name = "mxGetLogicals_800"]
    pub fn mxGetLogicals(pa: *const mxArray) -> *mut mxLogical;
    #[link_name = "mxCreateLogicalArray_800"]
    pub fn mxCreateLogicalArray(ndim: mwSize, dims: *const mwSize) -> *mut mxArray;
    #[link_name = "mxCreateLogicalMatrix_800"]
    pub fn mxCreateLogicalMatrix(m: mwSize, n: mwSize) -> *mut mxArray;
    #[link_name = "mxCreateLogicalScalar_800"]
    pub fn mxCreateLogicalScalar(value: bool) -> *mut mxArray;
    #[link_name = "mxIsLogicalScalar_800"]
    pub fn mxIsLogicalScalar(pa: *const mxArray) -> bool;
    #[link_name = "mxIsLogicalScalarTrue_800"]
    pub fn mxIsLogicalScalarTrue(pa: *const mxArray) -> bool;
    #[link_name = "mxCreateDoubleScalar_800"]
    pub fn mxCreateDoubleScalar(value: f64) -> *mut mxArray;
    #[link_name = "mxCreateSparse_800"]
    pub fn mxCreateSparse(
        m: mwSize,
        n: mwSize,
        nzmax: mwSize,
        flag: mxComplexity::Type,
    ) -> *mut mxArray;
    #[link_name = "mxCreateSparseLogicalMatrix_800"]
    pub fn mxCreateSparseLogicalMatrix(m: mwSize, n: mwSize, nzmax: mwSize) -> *mut mxArray;
    #[link_name = "mxGetNChars_800"]
    pub fn mxGetNChars(pa: *const mxArray, buf: *mut ::core::ffi::c_char, nChars: mwSize);
    #[link_name = "mxGetString_800"]
    pub fn mxGetString(
        pa: *const mxArray,
        buf: *mut ::core::ffi::c_char,
        buflen: mwSize,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxArrayToString_800"]
    pub fn mxArrayToString(pa: *const mxArray) -> *mut ::core::ffi::c_char;
    #[link_name = "mxArrayToUTF8String_800"]
    pub fn mxArrayToUTF8String(pa: *const mxArray) -> *mut ::core::ffi::c_char;
    #[link_name = "mxCreateStringFromNChars_800"]
    pub fn mxCreateStringFromNChars(str_: *const ::core::ffi::c_char, n: mwSize) -> *mut mxArray;
    #[link_name = "mxCreateString_800"]
    pub fn mxCreateString(str_: *const ::core::ffi::c_char) -> *mut mxArray;
    #[link_name = "mxCreateCharMatrixFromStrings_800"]
    pub fn mxCreateCharMatrixFromStrings(
        m: mwSize,
        str_: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "mxCreateCellMatrix_800"]
    pub fn mxCreateCellMatrix(m: mwSize, n: mwSize) -> *mut mxArray;
    #[link_name = "mxCreateCellArray_800"]
    pub fn mxCreateCellArray(ndim: mwSize, dims: *const mwSize) -> *mut mxArray;
    #[link_name = "mxCreateStructMatrix_800"]
    pub fn mxCreateStructMatrix(
        m: mwSize,
        n: mwSize,
        nfields: ::core::ffi::c_int,
        fieldnames: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "mxCreateStructArray_800"]
    pub fn mxCreateStructArray(
        ndim: mwSize,
        dims: *const mwSize,
        nfields: ::core::ffi::c_int,
        fieldnames: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "mxDuplicateArray_800"]
    pub fn mxDuplicateArray(in_: *const mxArray) -> *mut mxArray;
    #[link_name = "mxSetClassName_800"]
    pub fn mxSetClassName(
        pa: *mut mxArray,
        classname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxAddField_800"]
    pub fn mxAddField(
        pa: *mut mxArray,
        fieldname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxRemoveField_800"]
    pub fn mxRemoveField(pa: *mut mxArray, field: ::core::ffi::c_int);
    #[link_name = "mxGetEps_800"]
    pub fn mxGetEps() -> f64;
    #[link_name = "mxGetInf_800"]
    pub fn mxGetInf() -> f64;
    #[link_name = "mxGetNaN_800"]
    pub fn mxGetNaN() -> f64;
    #[link_name = "mxIsFinite_800"]
    pub fn mxIsFinite(x: f64) -> bool;
    #[link_name = "mxIsInf_800"]
    pub fn mxIsInf(x: f64) -> bool;
    #[link_name = "mxIsNaN_800"]
    pub fn mxIsNaN(x: f64) -> bool;
    #[link_name = "mxGetDoubles_800"]
    pub fn mxGetDoubles(arg1: *const mxArray) -> *mut f64;
    #[link_name = "mxSetDoubles_800"]
    pub fn mxSetDoubles(arg1: *mut mxArray, arg2: *mut f64) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexDoubles_800"]
    pub fn mxGetComplexDoubles(arg1: *const mxArray) -> *mut mxComplexDouble;
    #[link_name = "mxSetComplexDoubles_800"]
    pub fn mxSetComplexDoubles(
        arg1: *mut mxArray,
        arg2: *mut mxComplexDouble,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxGetSingles_800"]
    pub fn mxGetSingles(arg1: *const mxArray) -> *mut f32;
    #[link_name = "mxSetSingles_800"]
    pub fn mxSetSingles(arg1: *mut mxArray, arg2: *mut f32) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexSingles_800"]
    pub fn mxGetComplexSingles(arg1: *const mxArray) -> *mut mxComplexSingle;
    #[link_name = "mxSetComplexSingles_800"]
    pub fn mxSetComplexSingles(
        arg1: *mut mxArray,
        arg2: *mut mxComplexSingle,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxGetInt8s_800"]
    pub fn mxGetInt8s(arg1: *const mxArray) -> *mut i8;
    #[link_name = "mxSetInt8s_800"]
    pub fn mxSetInt8s(arg1: *mut mxArray, arg2: *mut i8) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexInt8s_800"]
    pub fn mxGetComplexInt8s(arg1: *const mxArray) -> *mut mxComplexInt8;
    #[link_name = "mxSetComplexInt8s_800"]
    pub fn mxSetComplexInt8s(arg1: *mut mxArray, arg2: *mut mxComplexInt8) -> ::core::ffi::c_int;
    #[link_name = "mxGetUint8s_800"]
    pub fn mxGetUint8s(arg1: *const mxArray) -> *mut u8;
    #[link_name = "mxSetUint8s_800"]
    pub fn mxSetUint8s(arg1: *mut mxArray, arg2: *mut u8) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexUint8s_800"]
    pub fn mxGetComplexUint8s(arg1: *const mxArray) -> *mut mxComplexUint8;
    #[link_name = "mxSetComplexUint8s_800"]
    pub fn mxSetComplexUint8s(arg1: *mut mxArray, arg2: *mut mxComplexUint8) -> ::core::ffi::c_int;
    #[link_name = "mxGetInt16s_800"]
    pub fn mxGetInt16s(arg1: *const mxArray) -> *mut i16;
    #[link_name = "mxSetInt16s_800"]
    pub fn mxSetInt16s(arg1: *mut mxArray, arg2: *mut i16) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexInt16s_800"]
    pub fn mxGetComplexInt16s(arg1: *const mxArray) -> *mut mxComplexInt16;
    #[link_name = "mxSetComplexInt16s_800"]
    pub fn mxSetComplexInt16s(arg1: *mut mxArray, arg2: *mut mxComplexInt16) -> ::core::ffi::c_int;
    #[link_name = "mxGetUint16s_800"]
    pub fn mxGetUint16s(arg1: *const mxArray) -> *mut u16;
    #[link_name = "mxSetUint16s_800"]
    pub fn mxSetUint16s(arg1: *mut mxArray, arg2: *mut u16) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexUint16s_800"]
    pub fn mxGetComplexUint16s(arg1: *const mxArray) -> *mut mxComplexUint16;
    #[link_name = "mxSetComplexUint16s_800"]
    pub fn mxSetComplexUint16s(
        arg1: *mut mxArray,
        arg2: *mut mxComplexUint16,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxGetInt32s_800"]
    pub fn mxGetInt32s(arg1: *const mxArray) -> *mut i32;
    #[link_name = "mxSetInt32s_800"]
    pub fn mxSetInt32s(arg1: *mut mxArray, arg2: *mut i32) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexInt32s_800"]
    pub fn mxGetComplexInt32s(arg1: *const mxArray) -> *mut mxComplexInt32;
    #[link_name = "mxSetComplexInt32s_800"]
    pub fn mxSetComplexInt32s(arg1: *mut mxArray, arg2: *mut mxComplexInt32) -> ::core::ffi::c_int;
    #[link_name = "mxGetUint32s_800"]
    pub fn mxGetUint32s(arg1: *const mxArray) -> *mut u32;
    #[link_name = "mxSetUint32s_800"]
    pub fn mxSetUint32s(arg1: *mut mxArray, arg2: *mut u32) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexUint32s_800"]
    pub fn mxGetComplexUint32s(arg1: *const mxArray) -> *mut mxComplexUint32;
    #[link_name = "mxSetComplexUint32s_800"]
    pub fn mxSetComplexUint32s(
        arg1: *mut mxArray,
        arg2: *mut mxComplexUint32,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxGetInt64s_800"]
    pub fn mxGetInt64s(arg1: *const mxArray) -> *mut i64;
    #[link_name = "mxSetInt64s_800"]
    pub fn mxSetInt64s(arg1: *mut mxArray, arg2: *mut i64) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexInt64s_800"]
    pub fn mxGetComplexInt64s(arg1: *const mxArray) -> *mut mxComplexInt64;
    #[link_name = "mxSetComplexInt64s_800"]
    pub fn mxSetComplexInt64s(arg1: *mut mxArray, arg2: *mut mxComplexInt64) -> ::core::ffi::c_int;
    #[link_name = "mxGetUint64s_800"]
    pub fn mxGetUint64s(arg1: *const mxArray) -> *mut u64;
    #[link_name = "mxSetUint64s_800"]
    pub fn mxSetUint64s(arg1: *mut mxArray, arg2: *mut u64) -> ::core::ffi::c_int;
    #[link_name = "mxGetComplexUint64s_800"]
    pub fn mxGetComplexUint64s(arg1: *const mxArray) -> *mut mxComplexUint64;
    #[link_name = "mxSetComplexUint64s_800"]
    pub fn mxSetComplexUint64s(
        arg1: *mut mxArray,
        arg2: *mut mxComplexUint64,
    ) -> ::core::ffi::c_int;
    #[link_name = "mxMakeArrayReal_800"]
    pub fn mxMakeArrayReal(arg1: *mut mxArray) -> ::core::ffi::c_int;
    #[link_name = "mxMakeArrayComplex_800"]
    pub fn mxMakeArrayComplex(arg1: *mut mxArray) -> ::core::ffi::c_int;
    #[link_name = "mexErrMsgTxt_800"]
    pub fn mexErrMsgTxt(error_msg: *const ::core::ffi::c_char);
    #[link_name = "mexErrMsgIdAndTxt_800"]
    pub fn mexErrMsgIdAndTxt(
        identifier: *const ::core::ffi::c_char,
        err_msg: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "mexWarnMsgTxt_800"]
    pub fn mexWarnMsgTxt(warn_msg: *const ::core::ffi::c_char);
    #[link_name = "mexWarnMsgIdAndTxt_800"]
    pub fn mexWarnMsgIdAndTxt(
        identifier: *const ::core::ffi::c_char,
        warn_msg: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "mexPrintf_800"]
    pub fn mexPrintf(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name = "mexMakeArrayPersistent_800"]
    pub fn mexMakeArrayPersistent(pa: *mut mxArray);
    #[link_name = "mexMakeMemoryPersistent_800"]
    pub fn mexMakeMemoryPersistent(ptr: *mut ::core::ffi::c_void);
    #[link_name = "mexCallMATLAB_800"]
    pub fn mexCallMATLAB(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *mut mxArray,
        fcn_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "mexCallMATLABWithTrap_800"]
    pub fn mexCallMATLABWithTrap(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *mut mxArray,
        fcn_name: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "mexPrintAssertion_800"]
    pub fn mexPrintAssertion(
        test: *const ::core::ffi::c_char,
        fname: *const ::core::ffi::c_char,
        linenum: ::core::ffi::c_int,
        message: *const ::core::ffi::c_char,
    );
    #[link_name = "mexIsGlobal_800"]
    pub fn mexIsGlobal(pA: *const mxArray) -> bool;
    #[link_name = "mexPutVariable_800"]
    pub fn mexPutVariable(
        workspace: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        parray: *const mxArray,
    ) -> ::core::ffi::c_int;
    #[link_name = "mexGetVariablePtr_800"]
    pub fn mexGetVariablePtr(
        workspace: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
    ) -> *const mxArray;
    #[link_name = "mexGetVariable_800"]
    pub fn mexGetVariable(
        workspace: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "mexLock_800"]
    pub fn mexLock();
    #[link_name = "mexUnlock_800"]
    pub fn mexUnlock();
    #[link_name = "mexIsLocked_800"]
    pub fn mexIsLocked() -> bool;
    #[link_name = "mexFunctionName_800"]
    pub fn mexFunctionName() -> *const ::core::ffi::c_char;
    #[link_name = "mexEvalString_800"]
    pub fn mexEvalString(str_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "mexEvalStringWithTrap_800"]
    pub fn mexEvalStringWithTrap(str_: *const ::core::ffi::c_char) -> *mut mxArray;
    #[link_name = "mexAtExit_800"]
    pub fn mexAtExit(exit_fcn: mex_exit_fn) -> ::core::ffi::c_int;
    pub fn mexFunction(
        nlhs: ::core::ffi::c_int,
        plhs: *mut *mut mxArray,
        nrhs: ::core::ffi::c_int,
        prhs: *mut *const mxArray,
    );
    #[link_name = "matOpen_800"]
    pub fn matOpen(
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut MATFile;
    #[link_name = "matClose_800"]
    pub fn matClose(pMF: *mut MATFile) -> matError;
    #[link_name = "matGetErrno_800"]
    pub fn matGetErrno(pMF: *mut MATFile) -> matError;
    #[link_name = "matGetFp_800"]
    pub fn matGetFp(pMF: *mut MATFile) -> *mut FILE;
    #[link_name = "matPutVariable_800"]
    pub fn matPutVariable(
        pMF: *mut MATFile,
        name: *const ::core::ffi::c_char,
        pA: *const mxArray,
    ) -> matError;
    #[link_name = "matPutVariableAsGlobal_800"]
    pub fn matPutVariableAsGlobal(
        pMF: *mut MATFile,
        name: *const ::core::ffi::c_char,
        pA: *const mxArray,
    ) -> matError;
    #[link_name = "matGetVariable_800"]
    pub fn matGetVariable(pMF: *mut MATFile, name: *const ::core::ffi::c_char) -> *mut mxArray;
    #[link_name = "matGetNextVariable_800"]
    pub fn matGetNextVariable(
        pMF: *mut MATFile,
        nameptr: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "matGetNextVariableInfo_800"]
    pub fn matGetNextVariableInfo(
        pMF: *mut MATFile,
        nameptr: *mut *const ::core::ffi::c_char,
    ) -> *mut mxArray;
    #[link_name = "matGetVariableInfo_800"]
    pub fn matGetVariableInfo(pMF: *mut MATFile, name: *const ::core::ffi::c_char) -> *mut mxArray;
    #[link_name = "matDeleteVariable_800"]
    pub fn matDeleteVariable(pMF: *mut MATFile, name: *const ::core::ffi::c_char) -> matError;
    #[link_name = "matGetDir_800"]
    pub fn matGetDir(
        pMF: *mut MATFile,
        num: *mut ::core::ffi::c_int,
    ) -> *mut *mut ::core::ffi::c_char;
    pub fn engEvalString(ep: *mut Engine, string: *const ::core::ffi::c_char)
        -> ::core::ffi::c_int;
    pub fn engOpenSingleUse(
        startcmd: *const ::core::ffi::c_char,
        reserved: *mut ::core::ffi::c_void,
        retstatus: *mut ::core::ffi::c_int,
    ) -> *mut Engine;
    pub fn engSetVisible(ep: *mut Engine, newVal: bool) -> ::core::ffi::c_int;
    pub fn engGetVisible(ep: *mut Engine, bVal: *mut bool) -> ::core::ffi::c_int;
    pub fn engOpen(startcmd: *const ::core::ffi::c_char) -> *mut Engine;
    pub fn engClose(ep: *mut Engine) -> ::core::ffi::c_int;
    pub fn engGetVariable(ep: *mut Engine, name: *const ::core::ffi::c_char) -> *mut mxArray;
    pub fn engPutVariable(
        ep: *mut Engine,
        var_name: *const ::core::ffi::c_char,
        ap: *const mxArray,
    ) -> ::core::ffi::c_int;
    pub fn engOutputBuffer(
        ep: *mut Engine,
        buffer: *mut ::core::ffi::c_char,
        buflen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
