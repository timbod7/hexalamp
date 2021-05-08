#!/usr/bin/env python3

## Creates a gamma-corrected lookup table
## Thanks hackaday!

import math,os

def gamma(nsteps, gamma):
    gammaedUp = [math.pow(x, gamma) for x in range(nsteps)]
    return [x/max(gammaedUp) for x in gammaedUp]

def rounder(topValue, gammas):
    return [min(topValue, round(x*topValue)) for x in gammas]

if __name__ == "__main__":
    outpath = os.path.join(os.path.dirname(os.path.dirname(__file__)), "src/animation/gamma.rs")
    myGamma = 2.3
    steps = 64
    output = open(outpath, "w")
    output.write("/* %d-step brightness table: gamma = %s*/ \n\n" % (steps, myGamma))
    output.write("pub const GAMMA : [u8; %d] = [\n" % steps)
    for value in rounder(255, gamma(steps, myGamma)):
        output.write("\t %d,\n" % value)
    output.write("];\n")
    output.close()
