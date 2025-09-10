
fn fnlFastRound(f: f32) -> i32 {
    if f >= 0 {
        return i32(f + 0.5);
    } else {
        return i32(f - 0.5);
    }
}

fn fnlHash3D(seed: i32, xPrimed: i32, yPrimed: i32, zPrimed: i32) -> i32 {
    var hash = seed ^ xPrimed ^ yPrimed ^ zPrimed;
    hash *= 0x27d4eb2d;
    return hash;
}

var<private> GRADIENTS_3D: array<f32, 256> = array(
    0.0f, 1.0f, 1.0f, 0.0f,  0.0f, -1.0f, 1.0f, 0.0f,  0.0f,  1.0f, -1.0f, 0.0f,  0.0f, -1.0f, -1.0f, 0.0f,
    1.0f, 0.0f, 1.0f, 0.0f, -1.0f,  0.0f, 1.0f, 0.0f,  1.0f,  0.0f, -1.0f, 0.0f, -1.0f,  0.0f, -1.0f, 0.0f,
    1.0f, 1.0f, 0.0f, 0.0f, -1.0f,  1.0f, 0.0f, 0.0f,  1.0f, -1.0f,  0.0f, 0.0f, -1.0f, -1.0f,  0.0f, 0.0f,
    0.0f, 1.0f, 1.0f, 0.0f,  0.0f, -1.0f, 1.0f, 0.0f,  0.0f,  1.0f, -1.0f, 0.0f,  0.0f, -1.0f, -1.0f, 0.0f,
    1.0f, 0.0f, 1.0f, 0.0f, -1.0f,  0.0f, 1.0f, 0.0f,  1.0f,  0.0f, -1.0f, 0.0f, -1.0f,  0.0f, -1.0f, 0.0f,
    1.0f, 1.0f, 0.0f, 0.0f, -1.0f,  1.0f, 0.0f, 0.0f,  1.0f, -1.0f,  0.0f, 0.0f, -1.0f, -1.0f,  0.0f, 0.0f,
    0.0f, 1.0f, 1.0f, 0.0f,  0.0f, -1.0f, 1.0f, 0.0f,  0.0f,  1.0f, -1.0f, 0.0f,  0.0f, -1.0f, -1.0f, 0.0f,
    1.0f, 0.0f, 1.0f, 0.0f, -1.0f,  0.0f, 1.0f, 0.0f,  1.0f,  0.0f, -1.0f, 0.0f, -1.0f,  0.0f, -1.0f, 0.0f,
    1.0f, 1.0f, 0.0f, 0.0f, -1.0f,  1.0f, 0.0f, 0.0f,  1.0f, -1.0f,  0.0f, 0.0f, -1.0f, -1.0f,  0.0f, 0.0f,
    0.0f, 1.0f, 1.0f, 0.0f,  0.0f, -1.0f, 1.0f, 0.0f,  0.0f,  1.0f, -1.0f, 0.0f,  0.0f, -1.0f, -1.0f, 0.0f,
    1.0f, 0.0f, 1.0f, 0.0f, -1.0f,  0.0f, 1.0f, 0.0f,  1.0f,  0.0f, -1.0f, 0.0f, -1.0f,  0.0f, -1.0f, 0.0f,
    1.0f, 1.0f, 0.0f, 0.0f, -1.0f,  1.0f, 0.0f, 0.0f,  1.0f, -1.0f,  0.0f, 0.0f, -1.0f, -1.0f,  0.0f, 0.0f,
    0.0f, 1.0f, 1.0f, 0.0f,  0.0f, -1.0f, 1.0f, 0.0f,  0.0f,  1.0f, -1.0f, 0.0f,  0.0f, -1.0f, -1.0f, 0.0f,
    1.0f, 0.0f, 1.0f, 0.0f, -1.0f,  0.0f, 1.0f, 0.0f,  1.0f,  0.0f, -1.0f, 0.0f, -1.0f,  0.0f, -1.0f, 0.0f,
    1.0f, 1.0f, 0.0f, 0.0f, -1.0f,  1.0f, 0.0f, 0.0f,  1.0f, -1.0f,  0.0f, 0.0f, -1.0f, -1.0f,  0.0f, 0.0f,
    1.0f, 1.0f, 0.0f, 0.0f,  0.0f, -1.0f, 1.0f, 0.0f, -1.0f,  1.0f,  0.0f, 0.0f,  0.0f, -1.0f, -1.0f, 0.0f
);

fn fnlGradCoord3D(seed: i32, xPrimed: i32, yPrimed: i32, zPrimed: i32, xd: f32, yd: f32, zd: f32) -> f32 {
    var hash = fnlHash3D(seed, xPrimed, yPrimed, zPrimed);
    hash ^= hash >> 15;
    hash &= i32(63) << 2;
    return xd * GRADIENTS_3D[hash] + yd * GRADIENTS_3D[hash | 1] + zd * GRADIENTS_3D[hash | 2];
}

fn fnlSingleOpenSimplex23D(seed_param: i32, x: f32, y: f32, z: f32) -> f32 {
    
    var seed = seed_param;

    let PRIME_X = 501125321;
    let PRIME_Y = 1136930381;
    let PRIME_Z = 1720413743;

    var i = fnlFastRound(x); 
    var j = fnlFastRound(y); 
    var k = fnlFastRound(z); 
    var x0 = x - f32(i);
    var y0 = y - f32(j);
    var z0 = z - f32(k);

    var xNSign = i32(-1.0 - x0) | 1;
    var yNSign = i32(-1.0 - y0) | 1;
    var zNSign = i32(-1.0 - z0) | 1;

    var ax0 = f32(xNSign) * -x0;
    var ay0 = f32(yNSign) * -y0;
    var az0 = f32(zNSign) * -z0;

    i *= PRIME_X;
    j *= PRIME_Y;
    k *= PRIME_Z;

    var value = 0.0;
    var a = (0.6 - x0 * x0) - (y0 * y0 + z0 * z0);    
    for(var l = 0; ; l++) {
        if a > 0 {
            value += (a * a) * (a * a) * fnlGradCoord3D(seed, i, j, k, x0, y0, z0);
        }

        var b = a + 1.0;
        var i1 = i;
        var j1 = j;
        var k1 = k;
        var x1 = x0;
        var y1 = y0;
        var z1 = z0;

        if ax0 >= ay0 && ax0 >= az0 {
            x1 += f32(xNSign);
            b -= f32(xNSign) * 2.0 * x1;
            i1 -= xNSign * PRIME_X;
        } else if ay0 > ax0 && ay0 >= az0 {
            y1 += f32(yNSign);
            b -= f32(yNSign) * 2.0 * y1;
            j1 -= yNSign * PRIME_Z;
        } else {
            z1 += f32(zNSign);
            b -= f32(zNSign) * 2.0 * z1;
            k1 -= zNSign * PRIME_Z;
        }

        if b > 0 {
            value += (b * b) * (b * b) * fnlGradCoord3D(seed, i1, j1, k1, x1, y1, z1);
        }

        if l == 1 {
            break;
        }

        ax0 = 0.5 - ax0;
        ay0 = 0.5 - ay0;
        az0 = 0.5 - az0;

        x0 = f32(xNSign) * ax0;
        y0 = f32(yNSign) * ay0;
        z0 = f32(zNSign) * az0;

        a += (0.75 - ax0) - (ay0 + az0);
        i += (xNSign >> 1) & PRIME_X;
        j += (yNSign >> 1) & PRIME_Y;
        k += (zNSign >> 1) & PRIME_Z;

        xNSign = -xNSign;
        yNSign = -yNSign;
        zNSign = -zNSign;

        seed = ~seed;
    }

    return value * 32.69428253173828125;
}