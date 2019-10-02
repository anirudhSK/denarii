/* Copyright 2019, Gurobi Optimization, LLC */

/* This example formulates and solves the following simple MIP model:

     maximize    x +   y + 2 z
     subject to  x + 2 y + 3 z <= 4
                 x +   y       >= 1
                 x, y, z binary
*/

#include <stdlib.h>
#include <stdio.h>
#include "gurobi_c.h"

int
main(int   argc,
     char *argv[])
{
  GRBenv   *env   = NULL;
  GRBmodel *model = NULL;
  int       error = 0;
  double    sol[4];
  int       ind[4];
  double    val[4];
  double    obj[4];
  char      vtype[4];
  int       optimstatus;
  double    objval;

  /* Create environment */

  error = GRBloadenv(&env, "mip1.log");
  if (error) goto QUIT;

  /* Create an empty model */

  error = GRBnewmodel(env, &model, "mip1", 0, NULL, NULL, NULL, NULL, NULL);
  if (error) goto QUIT;


  /* Add variables */

  error = GRBaddvar(model, 0, NULL, NULL, 0, 0.0, GRB_INFINITY,
                    GRB_BINARY, "var1");
  if (error) goto QUIT;

  error = GRBaddvar(model, 0, NULL, NULL, 0, 0.0, GRB_INFINITY,
                    GRB_BINARY, "var2");
  if (error) goto QUIT;

  error = GRBaddvar(model, 0, NULL, NULL, 0, 0.0, GRB_INFINITY,
                    GRB_BINARY, "var3");
  if (error) goto QUIT;

  /* Change objective sense to maximization */

  error = GRBsetintattr(model, GRB_INT_ATTR_MODELSENSE, GRB_MAXIMIZE);
  if (error) goto QUIT;

  /* Objective as a constraint: x + y + 2z - obj == 0 */
  error = GRBaddvar(model, 0, NULL, NULL, 1, 0.0, GRB_INFINITY,
                    GRB_INTEGER, "var4");
  if (error) goto QUIT;
  ind[0] = 0; ind[1] = 1; ind[2] = 2; ind[3] = 3;
  val[0] = 1; val[1] = 1; val[2] = 2; val[3] = -1;
  error = GRBaddconstr(model, 4, ind, val, GRB_EQUAL, 0.0, "cequal");
  if (error) goto QUIT;

  /* First constraint: x + 2 y + 3 z <= 4 */

  ind[0] = 0; ind[1] = 1; ind[2] = 2;
  val[0] = 1; val[1] = 2; val[2] = 3;

  error = GRBaddconstr(model, 3, ind, val, GRB_LESS_EQUAL, 4.0, "c0");
  if (error) goto QUIT;

  /* Second constraint: x + y >= 1 */

  ind[0] = 0; ind[1] = 1;
  val[0] = 1; val[1] = 1;

  error = GRBaddconstr(model, 2, ind, val, GRB_GREATER_EQUAL, 1.0, "c1");
  if (error) goto QUIT;

  /* Optimize model */

  error = GRBoptimize(model);
  if (error) goto QUIT;

  /* Write model to 'mip1.lp' */

  error = GRBwrite(model, "mip1.lp");
  if (error) goto QUIT;

  /* Capture solution information */

  error = GRBgetintattr(model, GRB_INT_ATTR_STATUS, &optimstatus);
  if (error) goto QUIT;

  error = GRBgetdblattr(model, GRB_DBL_ATTR_OBJVAL, &objval);
  if (error) goto QUIT;

  double x = 0.0;
  error = GRBgetdblattrelement(model, GRB_DBL_ATTR_X, 0, &x);
  if (error) goto QUIT;

  double y = 0.0;
  error = GRBgetdblattrelement(model, GRB_DBL_ATTR_X, 1, &y);
  if (error) goto QUIT;

  double z = 0.0;
  error = GRBgetdblattrelement(model, GRB_DBL_ATTR_X, 2, &z);
  if (error) goto QUIT;

  printf("\nOptimization complete\n");
  if (optimstatus == GRB_OPTIMAL) {
    printf("Optimal objective: %.4e\n", objval);

    printf("  x=%.0f, y=%.0f, z=%.0f\n", x, y, z);
  } else if (optimstatus == GRB_INF_OR_UNBD) {
    printf("Model is infeasible or unbounded\n");
  } else {
    printf("Optimization was stopped early\n");
  }

QUIT:

  /* Error reporting */

  if (error != 0) {
    printf("ERROR: %s\n", GRBgeterrormsg(env));
    exit(1);
  }

  /* Free model */

  GRBfreemodel(model);

  /* Free environment */

  GRBfreeenv(env);

  return 0;
}
