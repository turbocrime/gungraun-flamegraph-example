#[inline(always)]
pub fn work(iters: u64) -> u64 {
    let mut acc: u64 = 0;
    for i in 0..iters {
        acc = acc.wrapping_add(i.wrapping_mul(i ^ 0x55));
    }
    acc
}

#[inline(never)]
pub fn top(n: u64) -> u64 {
    let mut acc = work(n * 33);
    acc = acc.wrapping_add(a01_l(n));
    acc = acc.wrapping_add(a01_c(n));
    acc = acc.wrapping_add(a01_r(n));
    acc = acc.wrapping_add(b01_l(n));
    acc = acc.wrapping_add(b01_c(n));
    acc = acc.wrapping_add(b01_r(n));
    acc
}
#[inline(never)]
fn a01_l(n: u64) -> u64 {
    work(n * 10)
}
#[inline(never)]
fn a01_r(n: u64) -> u64 {
    work(n * 1000)
}
#[inline(never)]
fn b01_l(n: u64) -> u64 {
    work(n * 1012)
}
#[inline(never)]
fn b01_r(n: u64) -> u64 {
    work(n * 22)
}

// Peak A: 13 frames deep (top -> a01_2 -> a02_2 -> ... -> a12_2)
#[inline(never)]
fn a01_c(n: u64) -> u64 {
    let mut acc = work(n * 34);
    acc = acc.wrapping_add(a02_l(n));
    acc = acc.wrapping_add(a02_c(n));
    acc = acc.wrapping_add(a02_r(n));
    acc
}
#[inline(never)]
fn a02_l(n: u64) -> u64 {
    work(n * 11)
}
#[inline(never)]
fn a02_r(n: u64) -> u64 {
    work(n * 1001)
}

#[inline(never)]
fn a02_c(n: u64) -> u64 {
    let mut acc = work(n * 35);
    acc = acc.wrapping_add(a03_l(n));
    acc = acc.wrapping_add(a03_c(n));
    acc = acc.wrapping_add(a03_r(n));
    acc
}
#[inline(never)]
fn a03_l(n: u64) -> u64 {
    work(n * 12)
}
#[inline(never)]
fn a03_r(n: u64) -> u64 {
    work(n * 1002)
}

#[inline(never)]
fn a03_c(n: u64) -> u64 {
    let mut acc = work(n * 36);
    acc = acc.wrapping_add(a04_l(n));
    acc = acc.wrapping_add(a04_c(n));
    acc = acc.wrapping_add(a04_r(n));
    acc
}
#[inline(never)]
fn a04_l(n: u64) -> u64 {
    work(n * 13)
}
#[inline(never)]
fn a04_r(n: u64) -> u64 {
    work(n * 1003)
}

#[inline(never)]
fn a04_c(n: u64) -> u64 {
    let mut acc = work(n * 37);
    acc = acc.wrapping_add(a05_l(n));
    acc = acc.wrapping_add(a05_c(n));
    acc = acc.wrapping_add(a05_r(n));
    acc
}
#[inline(never)]
fn a05_l(n: u64) -> u64 {
    work(n * 14)
}
#[inline(never)]
fn a05_r(n: u64) -> u64 {
    work(n * 1004)
}

#[inline(never)]
fn a05_c(n: u64) -> u64 {
    let mut acc = work(n * 38);
    acc = acc.wrapping_add(a06_l(n));
    acc = acc.wrapping_add(a06_c(n));
    acc = acc.wrapping_add(a06_r(n));
    acc
}
#[inline(never)]
fn a06_l(n: u64) -> u64 {
    work(n * 15)
}
#[inline(never)]
fn a06_r(n: u64) -> u64 {
    work(n * 1005)
}

#[inline(never)]
fn a06_c(n: u64) -> u64 {
    let mut acc = work(n * 39);
    acc = acc.wrapping_add(a07_l(n));
    acc = acc.wrapping_add(a07_c(n));
    acc = acc.wrapping_add(a07_r(n));
    acc
}
#[inline(never)]
fn a07_l(n: u64) -> u64 {
    work(n * 16)
}
#[inline(never)]
fn a07_r(n: u64) -> u64 {
    work(n * 1006)
}

#[inline(never)]
fn a07_c(n: u64) -> u64 {
    let mut acc = work(n * 40);
    acc = acc.wrapping_add(a08_l(n));
    acc = acc.wrapping_add(a08_c(n));
    acc = acc.wrapping_add(a08_r(n));
    acc
}
#[inline(never)]
fn a08_l(n: u64) -> u64 {
    work(n * 17)
}
#[inline(never)]
fn a08_r(n: u64) -> u64 {
    work(n * 1007)
}

#[inline(never)]
fn a08_c(n: u64) -> u64 {
    let mut acc = work(n * 41);
    acc = acc.wrapping_add(a09_l(n));
    acc = acc.wrapping_add(a09_c(n));
    acc = acc.wrapping_add(a09_r(n));
    acc
}
#[inline(never)]
fn a09_l(n: u64) -> u64 {
    work(n * 18)
}
#[inline(never)]
fn a09_r(n: u64) -> u64 {
    work(n * 1008)
}

#[inline(never)]
fn a09_c(n: u64) -> u64 {
    let mut acc = work(n * 42);
    acc = acc.wrapping_add(a10_l(n));
    acc = acc.wrapping_add(a10_c(n));
    acc = acc.wrapping_add(a10_r(n));
    acc
}
#[inline(never)]
fn a10_l(n: u64) -> u64 {
    work(n * 19)
}
#[inline(never)]
fn a10_r(n: u64) -> u64 {
    work(n * 1009)
}

#[inline(never)]
fn a10_c(n: u64) -> u64 {
    let mut acc = work(n * 43);
    acc = acc.wrapping_add(a11_l(n));
    acc = acc.wrapping_add(a11_c(n));
    acc = acc.wrapping_add(a11_r(n));
    acc
}
#[inline(never)]
fn a11_l(n: u64) -> u64 {
    work(n * 20)
}
#[inline(never)]
fn a11_r(n: u64) -> u64 {
    work(n * 1010)
}

#[inline(never)]
fn a11_c(n: u64) -> u64 {
    let mut acc = work(n * 44);
    acc = acc.wrapping_add(a12_l(n));
    acc = acc.wrapping_add(a12_c(n));
    acc = acc.wrapping_add(a12_r(n));
    acc
}
#[inline(never)]
fn a12_l(n: u64) -> u64 {
    work(n * 21)
}
#[inline(never)]
fn a12_c(n: u64) -> u64 {
    work(n * 100)
}
#[inline(never)]
fn a12_r(n: u64) -> u64 {
    work(n * 1011)
}

// Peak B: 12 frames deep (top -> b01_2 -> b02_2 -> ... -> b11_2)
#[inline(never)]
fn b01_c(n: u64) -> u64 {
    let mut acc = work(n * 45);
    acc = acc.wrapping_add(b02_l(n));
    acc = acc.wrapping_add(b02_c(n));
    acc = acc.wrapping_add(b02_r(n));
    acc
}
#[inline(never)]
fn b02_l(n: u64) -> u64 {
    work(n * 1013)
}
#[inline(never)]
fn b02_r(n: u64) -> u64 {
    work(n * 23)
}

#[inline(never)]
fn b02_c(n: u64) -> u64 {
    let mut acc = work(n * 46);
    acc = acc.wrapping_add(b03_l(n));
    acc = acc.wrapping_add(b03_c(n));
    acc = acc.wrapping_add(b03_r(n));
    acc
}
#[inline(never)]
fn b03_l(n: u64) -> u64 {
    work(n * 1014)
}
#[inline(never)]
fn b03_r(n: u64) -> u64 {
    work(n * 24)
}

#[inline(never)]
fn b03_c(n: u64) -> u64 {
    let mut acc = work(n * 47);
    acc = acc.wrapping_add(b04_l(n));
    acc = acc.wrapping_add(b04_c(n));
    acc = acc.wrapping_add(b04_r(n));
    acc
}
#[inline(never)]
fn b04_l(n: u64) -> u64 {
    work(n * 1015)
}
#[inline(never)]
fn b04_r(n: u64) -> u64 {
    work(n * 25)
}

#[inline(never)]
fn b04_c(n: u64) -> u64 {
    let mut acc = work(n * 48);
    acc = acc.wrapping_add(b05_l(n));
    acc = acc.wrapping_add(b05_c(n));
    acc = acc.wrapping_add(b05_r(n));
    acc
}
#[inline(never)]
fn b05_l(n: u64) -> u64 {
    work(n * 1016)
}
#[inline(never)]
fn b05_r(n: u64) -> u64 {
    work(n * 26)
}

#[inline(never)]
fn b05_c(n: u64) -> u64 {
    let mut acc = work(n * 49);
    acc = acc.wrapping_add(b06_l(n));
    acc = acc.wrapping_add(b06_c(n));
    acc = acc.wrapping_add(b06_r(n));
    acc
}
#[inline(never)]
fn b06_l(n: u64) -> u64 {
    work(n * 1017)
}
#[inline(never)]
fn b06_r(n: u64) -> u64 {
    work(n * 27)
}

#[inline(never)]
fn b06_c(n: u64) -> u64 {
    let mut acc = work(n * 50);
    acc = acc.wrapping_add(b07_l(n));
    acc = acc.wrapping_add(b07_c(n));
    acc = acc.wrapping_add(b07_r(n));
    acc
}
#[inline(never)]
fn b07_l(n: u64) -> u64 {
    work(n * 1018)
}
#[inline(never)]
fn b07_r(n: u64) -> u64 {
    work(n * 28)
}

#[inline(never)]
fn b07_c(n: u64) -> u64 {
    let mut acc = work(n * 51);
    acc = acc.wrapping_add(b08_l(n));
    acc = acc.wrapping_add(b08_c(n));
    acc = acc.wrapping_add(b08_r(n));
    acc
}
#[inline(never)]
fn b08_l(n: u64) -> u64 {
    work(n * 1019)
}
#[inline(never)]
fn b08_r(n: u64) -> u64 {
    work(n * 29)
}

#[inline(never)]
fn b08_c(n: u64) -> u64 {
    let mut acc = work(n * 52);
    acc = acc.wrapping_add(b09_l(n));
    acc = acc.wrapping_add(b09_c(n));
    acc = acc.wrapping_add(b09_r(n));
    acc
}
#[inline(never)]
fn b09_l(n: u64) -> u64 {
    work(n * 1020)
}
#[inline(never)]
fn b09_r(n: u64) -> u64 {
    work(n * 30)
}

#[inline(never)]
fn b09_c(n: u64) -> u64 {
    let mut acc = work(n * 53);
    acc = acc.wrapping_add(b10_l(n));
    acc = acc.wrapping_add(b10_c(n));
    acc = acc.wrapping_add(b10_r(n));
    acc
}
#[inline(never)]
fn b10_l(n: u64) -> u64 {
    work(n * 1021)
}
#[inline(never)]
fn b10_r(n: u64) -> u64 {
    work(n * 31)
}

#[inline(never)]
fn b10_c(n: u64) -> u64 {
    let mut acc = work(n * 54);
    acc = acc.wrapping_add(b11_l(n));
    acc = acc.wrapping_add(b11_c(n));
    acc = acc.wrapping_add(b11_r(n));
    acc
}
#[inline(never)]
fn b11_l(n: u64) -> u64 {
    work(n * 1022)
}
#[inline(never)]
fn b11_c(n: u64) -> u64 {
    work(n * 101)
}
#[inline(never)]
fn b11_r(n: u64) -> u64 {
    work(n * 32)
}
