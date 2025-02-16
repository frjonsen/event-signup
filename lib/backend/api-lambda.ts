import { AssetHashType } from "aws-cdk-lib";
import { Architecture } from "aws-cdk-lib/aws-lambda";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";
import { Sentry } from "../sentry";
import { EventTable } from "../event-table";

export interface ApiLambdaProps {
  sentry: Sentry;
  eventTable: EventTable;
}

export class ApiLambda extends RustFunction {
  constructor(scope: Construct, props: ApiLambdaProps) {
    super(scope, "ApiLambda", {
      architecture: Architecture.ARM_64,
      manifestPath: "lib/backend/api/Cargo.toml",
      bundling: {
        assetHashType: AssetHashType.SOURCE,
      },
      environment: {
        SENTRY_DSN: props.sentry.backendDsn.stringValue,
        EVENT_TABLE_ARN: props.eventTable.tableArn,
      },
    });

    props.eventTable.grantReadWriteData(this);
  }
}
