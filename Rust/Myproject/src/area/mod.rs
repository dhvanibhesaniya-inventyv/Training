
use crate::common_struct::Area;

///  This function is takes the value as length and breath.
pub fn main() {
    let values = Area {
        length: 5.0,
        breath: 5.0,
    };

    values.area_rectangle();
    values.area_square();
    values.area_circle();
}