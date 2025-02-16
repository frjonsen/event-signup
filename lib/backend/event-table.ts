import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";
import * as consts from "../consts";

export class EventTable extends dynamodb.TableV2 {
  constructor(scope: Construct) {
    super(scope, "EventTable", {
      deletionProtection: true,
      pointInTimeRecoverySpecification: {
        pointInTimeRecoveryEnabled: true,
      },
      partitionKey: {
        name: consts.EventTable.PartitionKey,
        type: dynamodb.AttributeType.STRING,
      },
      sortKey: {
        name: consts.EventTable.DefaultSortingKey,
        type: dynamodb.AttributeType.STRING,
      },
    });

    this.addGlobalSecondaryIndex({
      partitionKey: {
        name: consts.EventTable.EventCreatorColumn,
        type: dynamodb.AttributeType.STRING,
      },
      indexName: "EventCreatorIndex",
    });
  }
}
