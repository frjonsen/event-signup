import { AssetHashType, Duration } from "aws-cdk-lib";
import { Architecture } from "aws-cdk-lib/aws-lambda";
import { RustFunction } from "cargo-lambda-cdk";
import { Construct } from "constructs";
import { Sentry } from "../sentry";
import { EventTable } from "./event-table";
import { EventImageStorage } from "./event-image-storage";
import { UserPool } from "../authentication/user-pool";

export interface ApiLambdaProps {
  sentry: Sentry;
  eventTable: EventTable;
  images: EventImageStorage;
  memory?: number;
}

export class ApiLambda extends RustFunction {
  constructor(scope: Construct, id: string, props: ApiLambdaProps) {
    super(scope, id, {
      architecture: Architecture.ARM_64,
      timeout: Duration.minutes(1),
      memorySize: props.memory ?? 128,
      manifestPath: "lib/backend/events-api",
      bundling: {
        assetHashType: AssetHashType.SOURCE,
      },
      environment: {
        CONTENT_CREATORS_GROUP_NAME: UserPool.CONTENT_CREATORS_GROUP_NAME,
        SENTRY_DSN: props.sentry.backendDsn.stringValue,
        EVENT_TABLE_ARN: props.eventTable.tableArn,
        EVENT_IMAGES_BUCKET_NAME: props.images.bucketName,
        EVENT_IMAGES_BUCKET_PREFIX: "static/events",
      },
    });

    props.images.grantWrite(this);
    props.eventTable.grantReadWriteData(this);
  }
}
