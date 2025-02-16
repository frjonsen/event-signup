import { AssetHashType, Duration } from "aws-cdk-lib";
import { Architecture } from "aws-cdk-lib/aws-lambda";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";
import { Sentry } from "../sentry";
import { EventTable } from "./event-table";
import { EventImageStorage } from "./event-image-storage";

export interface ApiLambdaProps {
  sentry: Sentry;
  eventTable: EventTable;
  images: EventImageStorage;
}

export class ApiLambda extends RustFunction {
  constructor(scope: Construct, props: ApiLambdaProps) {
    super(scope, "ApiLambda", {
      architecture: Architecture.ARM_64,
      timeout: Duration.minutes(1),
      memorySize: 2048,
      manifestPath: "lib/backend/events-api",
      bundling: {
        assetHashType: AssetHashType.SOURCE,
      },
      environment: {
        SENTRY_DSN: props.sentry.backendDsn.stringValue,
        EVENT_TABLE_ARN: props.eventTable.tableArn,
        EVENT_IMAGES_BUCKET_ARN: props.images.bucketArn,
      },
    });

    props.images.grantWrite(this);
    props.eventTable.grantReadWriteData(this);
  }
}
