# Denarii: Multi tenant support for SmartNICs

[![Build status](https://ci.appveyor.com/api/projects/status/qic0fwba87qnk6er/branch/master?svg=true)](https://ci.appveyor.com/project/anirudhSK/dinarii/branch/master)

# Install Gurobi

Follow instructions in [Gurobi Documentation](https://www.gurobi.com/documentation/quickstart.html)
to install and setup license key for your machine.

To compile `gurobi_example.c`, use:
```
gcc -m64 -I/opt/gurobi900/linux64/include/ gurobi_example.c -L /opt/gurobi900/linux64/lib/ -lgurobi90 -lm
```
