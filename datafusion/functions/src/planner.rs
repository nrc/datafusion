// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! SQL planning extensions like [`UserDefinedFunctionPlanner`]

use datafusion_common::Result;
use datafusion_expr::{
    expr::ScalarFunction,
    planner::{PlannerResult, UserDefinedSQLPlanner},
    Expr,
};

#[derive(Default)]
pub struct UserDefinedFunctionPlanner;

impl UserDefinedSQLPlanner for UserDefinedFunctionPlanner {
    #[cfg(feature = "datetime_expressions")]
    fn plan_extract(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> {
        Ok(PlannerResult::Planned(Expr::ScalarFunction(
            ScalarFunction::new_udf(crate::datetime::date_part(), args),
        )))
    }

    #[cfg(feature = "unicode_expressions")]
    fn plan_position(&self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> {
        Ok(PlannerResult::Planned(Expr::ScalarFunction(
            ScalarFunction::new_udf(crate::unicode::strpos(), args),
        )))
    }
}
