// Ajouter inline provoque un gain substentiel.
// RustC ne prends pas la liberté d'inline systématiquement.
pub fn is_leap_year(year: u64) -> bool {
    !is_not_leap_year(year)
}

#[inline]
pub fn is_not_leap_year(year: u64) -> bool {
    year % 4 != 0 || year % 400 != 0 && year % 100 == 0
}

pub fn is_leap_year_overcharged(y: u64) -> bool {
    (((y % 4 == 0) as u64) & ((y % 100 != 0) as u64) | ((y % 400 == 0) as u64)) == 0
}

pub fn is_leap_year_if(year: u64) -> bool {
    if year % 100 == 0 && year % 400 != 0 {
        return false;
    }

    if year % 4 == 0 {
        return true;
    }

    return false;
}

pub fn is_leap_year_if_not(year: u64) -> bool {
    if year % 4 != 0 {
        return false;
    }

    if year % 400 != 0 {
        return false;
    }

    if year % 100 == 0 {
        return false;
    }

    return true;
}

pub fn is_leap_year_group(year: u64) -> bool {
    let divisible_by = |n| year % n == 0;

    match (4 % n, 100 % n, 400 % n) {
        (0, 0, 0) => true,
        (0, 0, 0) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}
