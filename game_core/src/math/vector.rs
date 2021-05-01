#[repr(C)]
#[derive(Debug)]
pub struct Vector1<T> {
    pub x: T,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_vector {
    ($VectorN:ident { $($field:ident),+ }, $func_name:ident) => {
        impl<T> $VectorN<T> {
            #[inline]
            pub fn $func_name($($field: T),+ ) -> $VectorN<T> {
                $VectorN{ $($field: $field),+ }
            }
        }
    };
}

impl_vector!(Vector1 { x }, new);
impl_vector!(Vector2 { x, y }, new);
impl_vector!(Vector3 { x, y, z }, new);