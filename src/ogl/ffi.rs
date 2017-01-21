// OpenGL FFI

use libc::*;

/* Datatypes(for transparent interfacing) */
pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLvoid = c_void;
pub type GLbyte = i8;
pub type GLshort = i16;
pub type GLint = i32;
pub type GLubyte = u8;
pub type GLushort = u16;
pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;

/* Constants */
pub const GL_TRUE: GLboolean = 1;
pub const GL_FALSE: GLboolean = 0;

pub const GL_BYTE: GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_SHORT: GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_INT: GLenum = 0x1404;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_2_BYTES: GLenum = 0x1407;
pub const GL_3_BYTES: GLenum = 0x1408;
pub const GL_4_BYTES: GLenum = 0x1409;
pub const GL_DOUBLE: GLenum = 0x140a;

pub const GL_POINTS: GLenum = 0x0000;
pub const GL_LINES: GLenum = 0x0001;
pub const GL_LINE_LOOP: GLenum = 0x0002;
pub const GL_LINE_STRIP: GLenum = 0x0003;
pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
pub const GL_QUADS: GLenum = 0x0007;
pub const GL_QUAD_STRIP: GLenum = 0x0008;
pub const GL_POLYGON: GLenum = 0x0009;

pub const GL_VERTEX_ARRAY: GLenum = 0x8074;
pub const GL_NORMAL_ARRAY: GLenum = 0x8075;
pub const GL_COLOR_ARRAY: GLenum = 0x8076;
pub const GL_INDEX_ARRAY: GLenum = 0x8077;
pub const GL_TEXTURE_COORD_ARRAY: GLenum = 0x8078;
pub const GL_EDGE_FLAG_ARRAY: GLenum = 0x8079;
pub const GL_VERTEX_ARRAY_SIZE: GLenum = 0x807a;
pub const GL_VERTEX_ARRAY_TYPE: GLenum = 0x807b;
pub const GL_VERTEX_ARRAY_STRIDE: GLenum = 0x807c;
pub const GL_NORMAL_ARRAY_TYPE: GLenum = 0x807e;
pub const GL_NORMAL_ARRAY_STRIDE: GLenum = 0x807f;
pub const GL_COLOR_ARRAY_SIZE: GLenum = 0x8081;
pub const GL_COLOR_ARRAY_TYPE: GLenum = 0x8082;
pub const GL_COLOR_ARRAY_STRIDE: GLenum = 0x8083;
pub const GL_INDEX_ARRAY_TYPE: GLenum = 0x8085;
pub const GL_INDEX_ARRAY_STRIDE: GLenum = 0x8086;
pub const GL_TEXTURE_COORD_ARRAY_SIZE: GLenum = 0x8088;
pub const GL_TEXTURE_COORD_ARRAY_TYPE: GLenum = 0x8089;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE: GLenum = 0x808a;
pub const GL_EDGE_FLAG_ARRAY_STRIDE: GLenum = 0x808c;
pub const GL_VERTEX_ARRAY_POINTER: GLenum = 0x808e;
pub const GL_NORMAL_ARRAY_POINTER: GLenum = 0x808f;
pub const GL_COLOR_ARRAY_POINTER: GLenum = 0x8090;
pub const GL_INDEX_ARRAY_POINTER: GLenum = 0x8091;
pub const GL_TEXTURE_COORD_ARRAY_POINTER: GLenum = 0x8092;
pub const GL_EDGE_FLAG_ARRAY_POINTER: GLenum = 0x8093;
/** Omitted for Fixed Vertex Format Constants **/

pub const GL_MATRIX_MODE: GLenum = 0x0ba0;
pub const GL_MODELVIEW: GLenum = 0x1700;
pub const GL_PROJECTION: GLenum = 0x1701;
pub const GL_TEXTURE: GLenum = 0x1702;

pub const GL_POINT_SMOOTH: GLenum = 0x0b10;
pub const GL_POINT_SIZE: GLenum = 0x0b11;
pub const GL_POINT_SIZE_GRANULARITY: GLenum = 0x0b13;
pub const GL_POINT_SIZE_RANGE: GLenum = 0x0b12;

pub const GL_LINE_SMOOTH: GLenum = 0x0b20;
pub const GL_LINE_STRIPPLE: GLenum = 0x0b24;
pub const GL_LINE_STRIPPLE_PATTERN: GLenum = 0x0b25;
pub const GL_LINE_STRIPPLE_REPEAT: GLenum = 0x0b26;
pub const GL_LINE_WIDTH: GLenum = 0x0b21;
pub const GL_LINE_WIDTH_GRANULARITY: GLenum = 0x0b23;
pub const GL_LINE_WIDTH_RANGE: GLenum = 0x0b22;

pub const GL_POINT: GLenum = 0x1b00;
pub const GL_LINE: GLenum = 0x1b01;
pub const GL_FILL: GLenum = 0x1b02;
pub const GL_CW: GLenum = 0x0900;
pub const GL_CCW: GLenum = 0x0901;
pub const GL_FRONT: GLenum = 0x0404;
pub const GL_BACK: GLenum = 0x0405;
pub const GL_POLYGON_MODE: GLenum = 0x0b40;
pub const GL_POLYGON_SMOOTH: GLenum = 0x0b41;
pub cosnt GL_POLYGON_STRIPPLE: GLenum = 0x0b42;
pub const GL_EDGE_FLAG: GLenum = 0x0b43;
pub const GL_CULL_FACE: GLenum = 0x0b44;
pub const GL_CULL_FACE_MODE: GLenum = 0x0b45;
pub const GL_FRONT_FACE: GLenum = 0x0b46;
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const GL_POLYGON_OFFSET_UNITS: GLenum = 0x2a00;
pub const GL_POLYGON_OFFSET_POINT: GLenum = 0x2a01;
pub const GL_POLYGON_OFFSET_LINE: GLenum = 0x2a02;
pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;

/** Omitting for Display List Constants **/

pub const GL_NEVER: GLenum = 0x0200;
pub const GL_LESS: GLenum = 0x0201;
pub const GL_EQUAL: GLenum = 0x0202;
pub const GL_LEQUAL: GLenum = 0x0203;
pub const GL_GREATER: GLenum = 0x0204;
pub const GL_NOTEQUAL: GLenum = 0x0205;
pub const GL_GEQUAL: GLenum = 0x0206;
pub const GL_ALWAYS: GLenum = 0x0207;
pub const GL_DEPTH_TEST: GLenum = 0x0b71;
pub const GL_DEPTH_BITS: GLenum = 0x0d56;
pub const GL_DEPTH_CLEAR_VALUE: GLenum = 0x0b73;
pub const GL_DEPTH_FUNC: GLenum = 0x0b74;
pub const GL_DEPTH_RANGE: GLenum = 0x0b70;
pub const GL_DEPTH_WRITEMASK: GLenum = 0x0b72;
pub const GL_DEPTH_COMPONENT: GLenum = 0x1902;

/** Omitting for Lighting Constants **/
/** Omitting for Clipping Plane Constants **/

pub const GL_ACCUM_RED_BITS: GLenum = 0x0d58;
pub const GL_ACCUM_GREEN_BITS: GLenum = 0x0d59;
pub const GL_ACCUM_BLUE_BITS: GLenum = 0x0d5a;
pub const GL_ACCUM_ALPHA_BITS: GLenum = 0x0d5b;
pub const GL_ACCUM_CLEAR_VALUE: GLenum = 0x0b80;
pub const GL_ACCUM: GLenum = 0x0100;
pub const GL_ADD: GLenum = 0x0104;
pub const GL_LOAD: GLenum = 0x0101;
pub const GL_MULT: GLenum = 0x0103;
pub const GL_RETURN: GLenum = 0x0102;

pub const GL_ALPHA_TEST: GLenum = 0x0bc0;
pub const GL_ALPHA_TEST_REF: GLenum = 0x0bc2;
pub const GL_ALPHA_TEST_FUNC: GLenum = 0x0bc1;

pub const GL_BLEND: GLenum = 0x0be2;
pub const GL_BLEND_SRC: GLenum = 0x0be1;
pub const GL_BLEND_DST: GLenum = 0x0be0;
pub const GL_ZERO: GLenum = 0;
pub const GL_ONE: GLenum = 1;
pub const GL_SRC_COLOR: GLenum = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const GL_SRC_ALPHA: GLenum = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const GL_DST_ALPHA: GLenum = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const GL_DST_COLOR: GLenum = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;

pub const GL_FEEDBACK: GLenum = 0x1c01;
pub const GL_RENDER: GLenum = 0x1c00;
pub const GL_SELECT: GLenum = 0x1c02;

/** Omitted for Feedback Buffer Constants **/
/** Omitted for Selection Buffer Constants **/
/** Omitted for Fog Constants **/

pub const GL_LOGIC_OP: GLenum = 0x0bf1;
pub const GL_INDEX_LOGIC_OP: GLenum = 0x0bf1;
pub const GL_COLOR_LOGIC_OP: GLenum = 0x0bf2;
pub const GL_LOGIC_OP_MODE: GLenum = 0x0bf0;
pub const GL_CLEAR: GLenum = 0x1500;
pub const GL_SET: GLenum = 0x150f;
pub const GL_COPY: GLenum = 0x1503;
pub const GL_COPY_INVERTED: GLenum = 0x150c;
pub const GL_NOOP: GLenum = 0x1505;
pub const GL_INVERT: GLenum = 0x150a;
pub const GL_AND: GLenum = 0x1501;
pub const GL_NAND: GLenum = 0x150e;
pub const GL_OR: GLenum = 0x1507;
pub const GL_NOR: GLenum = 0x1508;
pub const GL_XOR: GLenum = 0x1506;
pub const GL_EQUIV: GLenum = 0x1509;
pub const GL_AND_REVERSE: GLenum = 0x1502;
pub const GL_AND_INVERTED: GLenum = 0x1504;
pub const GL_OR_REVERSE: GLenum = 0x150b;
pub const GL_OR_INVERTED: GLenum = 0x150d;

pub const GL_STENCIL_BITS: GLenum = 0x0d57;
pub const GL_STENCIL_TEST: GLenum = 0x0b90;
pub const GL_STENCIL_CLEAR_VALUE: GLenum = 0x0b91;
pub const GL_STENCIL_FUNC: GLenum = 0x0b92;
pub const GL_STENCIL_VALUE_MASK: GLenum = 0x0b93;
pub const GL_STENCIL_FAIL: GLenum = 0x0b94;
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0b95;
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0b96;
pub const GL_STENCIL_REF: GLenum = 0x0b97;
pub const GL_STENCIL_WRITEMASK: GLenum = 0x0b98;
pub const GL_STENCIL_INDEX: GLenum = 0x1901;
pub const GL_KEEP: GLenum = 0x1e00;
pub const GL_REPLACE: GLenum = 0x1e01;
pub const GL_INCR: GLenum = 0x1e02;
pub const GL_DECR: GLenum = 0x1e03;

pub const GL_NONE: GLenum = 0;
pub const GL_LEFT: GLenum = 0x0406;
pub const GL_RIGHT: GLenum = 0x0407;
pub const GL_FRONT_LEFT: GLenum = 0x0400;
pub const GL_FRONT_RIGHT: GLenum = 0x0401;
pub const GL_BACK_LEFT: GLenum = 0x0402;
pub const GL_BACK_RIGHT: GLenum = 0x0403;
pub const GL_AUX0: GLenum = 0x0409;
pub const GL_AUX1: GLenum = 0x040a;
pub const GL_AUX2: GLenum = 0x040b;
pub const GL_AUX3: GLenum = 0x040c;
pub const GL_COLOR_INDEX: GLenum = 0x1900;
pub const GL_RED: GLenum = 0x1903;
pub const GL_GREEN: GLenum = 0x1904;
pub const GL_BLUE: GLenum = 0x1905;
pub const GL_ALPHA: GLenum = 0x1906;
pub const GL_LUMINANCE: GLenum = 0x1909;
pub const GL_LUMINANCE_ALPHA: GLenum = 0x190a;
pub const GL_ALPHA_BITS: GLenum = 0x0d55;
pub const GL_RED_BITS: GLenum = 0x0d52;
pub const GL_GREEN_BITS: GLenum = 0x0d53;
pub const GL_BLUE_BITS: GLenum = 0x0d54;
pub const GL_INDEX_BITS: GLenum = 0x0d51;
pub const GL_SUBPIXEL_BITS: GLenum = 0x0d50;
pub const GL_AUX_BUFFERS: GLenum = 0x0c00;
pub const GL_READ_BUFFER: GLenum = 0x0c02;
pub const GL_DRAW_BUFFER: GLenum = 0x0c01;
pub const GL_DOUBLEBUFFER: GLenum = 0x0c32;
pub const GL_STEREO: GLenum = 0x0c33;
pub const GL_BITMAP: GLenum = 0x1a00;
pub const GL_COLOR: GLenum = 0x1800;
pub const GL_DEPTH: GLenum = 0x1801;
pub const GL_STENCIL: GLenum = 0x1802;
pub const GL_DITHER: GLenum = 0x0bd0;
pub const GL_RGB: GLenum = 0x1907;
pub const GL_RGBA: GLenum = 0x1908;

pub const GL_MAX_LIST_NESTING: GLenum = 0x0b31;
pub const GL_MAX_EVAL_ORDER: GLenum = 0x0d30;
pub const GL_MAX_LIGHTS: GLenum = 0x0d31;
pub const GL_MAX_CLIP_PLANES: GLenum = 0x0d32;
pub const GL_MAX_TEXTURE_SIZE: GLenum = 0x0d33;
pub const GL_MAX_PIXEL_MAP_TABLE: GLenum = 0x0d34;
pub const GL_MAX_ATTRIB_STACK_DEPTH: GLenum = 0x0d35;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: GLenum = 0x0d36;
pub const GL_MAX_NAME_STACK_DEPTH: GLenum = 0x0d37;
pub const GL_MAX_PROJECTION_STACK_DEPTH: GLenum = 0x0d38;
pub const GL_MAX_TEXTURE_STACK_DEPTH: GLenum = 0x0d39;
pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0d3a;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0d3b;

/** Omitting for Getting Constants **/
/** Omitting for Evaluator Constants **/
/** Omitting for Hint Constants **/

pub const GL_SCISSOR_BOX: GLenum = 0x0c10;
pub const GL_SCISSOR_TEST: GLenum = 0x0c11;

pub const GL_MAP_COLOR: GLenum = 0x0d10;
pub const GL_MAP_STENCIL: GLenum = 0x0d11;
pub const GL_INDEX_SHIFT: GLenum = 0x0d12;
pub const GL_INDEX_OFFSET: GLenum = 0x0d13;
pub const GL_RED_SCALE: GLenum = 0x0d14;
pub const GL_RED_BIAS: GLenum = 0x0d15;
pub const GL_GREEN_SCALE: GLenum = 0x0d18;
pub const GL_GREEN_BIAS: GLenum = 0x0d19;
pub const GL_BLUE_SCALE: GLenum = 0x0d1a;
pub const GL_BLUE_BIAS: GLenum = 0x0d1b;
pub const GL_ALPHA_SCALE: GLenum = 0x0d1c;
pub const GL_ALPHA_BIAS: GLenum = 0x0d1d;
pub const GL_DEPTH_SCALE: GLenum = 0x0d1e;
pub cosnt GL_DEPTH_BIAS: GLenum = 0x0d1f;
pub const GL_PIXEL_MAP_S_TO_S_SIZE: GLenum = 0x0cb1;
pub const GL_PIXEL_MAP_I_TO_I_SIZE: GLenum = 0x0cb0;
pub const GL_PIXEL_MAP_I_TO_R_SIZE: GLenum = 0x0cb2;
pub const GL_PIXEL_MAP_I_TO_G_SIZE: GLenum = 0x0cb3;
pub const GL_PIXEL_MAP_I_TO_B_SIZE: GLenum = 0x0cb4;
pub const GL_PIXEL_MAP_I_TO_A_SIZE: GLenum = 0x0cb5;
pub const GL_PIXEL_MAP_R_TO_R_SIZE: GLenum = 0x0cb6;
pub const GL_PIXEL_MAP_G_TO_G_SIZE: GLenum = 0x0cb7;
pub const GL_PIXEL_MAP_B_TO_B_SIZE: GLenum = 0x0cb8;
pub const GL_PIXEL_MAP_A_TO_A_SIZE: GLenum = 0x0cb9;
pub const GL_PIXEL_MAP_S_TO_S: GLenum = 0x0c71;
pub const GL_PIXEL_MAP_I_TO_I: GLenum = 0x0c70;
pub const GL_PIXEL_MAP_I_TO_R: GLenum = 0x0c72;
pub const GL_PIXEL_MAP_I_TO_G: GLenum = 0x0c73;
pub const GL_PIXEL_MAP_I_TO_B: GLenum = 0x0c74;
pub const GL_PIXEL_MAP_I_TO_A: GLenum = 0x0c75;
pub const GL_PIXEL_MAP_R_TO_R: GLenum = 0x0c76;
pub const GL_PIXEL_MAP_G_TO_G: GLenum = 0x0c77;
pub const GL_PIXEL_MAP_B_TO_B: GLenum = 0x0c78;
pub const GL_PIXEL_MAP_A_TO_A: GLenum = 0x0c79;
pub const GL_PACK_ALIGNMENT: GLenum = 0x0d05;
pub const GL_PACK_LSB_FIRST: GLenum = 0x0d01;
pub const GL_PACK_ROW_LENGTH: GLenum = 0x0d02;
pub const GL_PACK_SKIP_PIXELS: GLenum = 0x0d04;
pub const GL_PACK_SKIP_ROWS: GLenum = 0x0d03;
pub const GL_PACK_SWAP_BYTES: GLenum = 0x0d00;
pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0cf5;
pub const GL_UNPACK_LSB_FIRST: GLenum = 0x0cf1;
pub const GL_UNPACK_ROW_LENGTH: GLenum = 0x0cf2;
pub const GL_UNPACK_SKIP_PIXELS: GLenum = 0x0cf4;
pub const GL_UNPACK_SKIP_ROWS: GLenum = 0x0cf3;
pub const GL_UNPACK_SWAP_BYTES: GLenum = 0x0cf0;
pub const GL_ZOOM_X: GLenum = 0x0d16;
pub const GL_ZOOM_Y: GLenum = 0x0d17;

pub const GL_TEXTURE_ENV: GLenum = 0x2300;
pub const GL_TEXTURE_ENV_MODE: GLenum = 0x2200;
pub const GL_TEXTURE_1D: GLenum = 0x0de0;
pub const GL_TEXTURE_2D: GLenum = 0x0de1;
pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const GL_TEXTURE_ENV_COLOR: GLenum = 0x2201;
pub const GL_TEXTURE_GEN_S: GLenum = 0x0c60;
pub const GL_TEXTURE_GEN_T: GLenum = 0x0c61;
pub const GL_TEXTURE_GEN_R: GLenum = 0x0c62;
pub const GL_TEXTURE_GEN_Q: GLenum = 0x0c63;
pub const GL_TEXTURE_GEN_MODE: GLenum = 0x2500;
pub const GL_TEXTURE_BORDER_MODE: GLenum = 0x1004;
pub const GL_TEXTURE_WIDTH: GLenum = 0x1000;
pub const GL_TEXTURE_HEIGHT: GLenum = 0x1001;
pub const GL_TEXTURE_BORDER: GLenum = 0x1005;
pub const GL_TEXTURE_COMPONENTS: GLenum = 0x1003;
pub const GL_TEXTURE_RED_SIZE: GLenum = 0x805c;
pub const GL_TEXTURE_GREEN_SIZE: GLenum = 0x805d;
pub const GL_TEXTURE_BLUE_SIZE: GLenum = 0x805e;
pub const GL_TEXTURE_ALPHA_SIZE: GLenum = 0x805f;
pub const GL_TEXTURE_LUMINANCE_SIZE: GLenum = 0x8060;
pub const GL_TEXTURE_INTENSITY_SIZE: GLenum = 0x8061;
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2701;
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2702;
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const GL_OBJECT_LINEAR: GLenum = 0x2401;
pub const GL_OBJECT_PLANE: GLenum = 0x2501;
pub const GL_EYE_LINEAR: GLenum = 0x2400;
pub const GL_EYE_PLANE: GLenum = 0x2502;
pub const GL_SPHERE_MAP: GLenum = 0x2402;
pub const GL_DECAL: GLenum = 0x2101;
pub const GL_MODULATE: GLenum = 0x2100;
pub const GL_NEAREST: GLenum = 0x2600;
pub const GL_REPEAT: GLenum = 0x2901;
pub const GL_CLAMP: GLenum = 0x2900;
pub const GL_S: GLenum = 0x2000;
pub const GL_T: GLenum = 0x2001;
pub const GL_R: GLenum = 0x2002;
pub const GL_Q: GLenum = 0x2003;

pub const GL_VENDOR: GLenum = 0x1f00;
pub const GL_RENDERER: GLenum = 0x1f01;
pub const GL_VERSION: GLenum = 0x1f02;
pub const GL_EXTENSIONS: GLenum = 0x1f03;

pub const GL_NO_ERROR: GLenum = 0;
pub const GL_INVALID_ENUM: GLenum = 0x0500;
pub const GL_INVALID_VALUE: GLenum = 0x0501;
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
pub const GL_STACK_OVERFLOW: GLenum = 0x0503;
pub const GL_STACK_UNDERFLOW: GLenum = 0x0504;
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;

/** Omitted for glPush/PopAttrib Bits **/
/** Omitted for OpenGL 1.1 Constants **/
