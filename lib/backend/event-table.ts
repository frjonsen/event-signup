import * as iam from "aws-cdk-lib/aws-iam";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";
import * as db from "./events-api/database-structure.json";

export class EventTable extends dynamodb.TableV2 {
  constructor(scope: Construct) {
    super(scope, "EventTable", {
      deletionProtection: true,
      pointInTimeRecoverySpecification: {
        pointInTimeRecoveryEnabled: true,
      },
      partitionKey: {
        name: db.partition_key_column,
        type: dynamodb.AttributeType.STRING,
      },
      sortKey: {
        name: db.sorting_key_column,
        type: dynamodb.AttributeType.STRING,
      },
    });

    this.addGlobalSecondaryIndex({
      partitionKey: {
        name: db.creator_column,
        type: dynamodb.AttributeType.STRING,
      },
      sortKey: {
        name: db.partition_key_column,
        type: dynamodb.AttributeType.STRING,
      },
      projectionType: dynamodb.ProjectionType.INCLUDE,
      nonKeyAttributes: [
        db.title_column,
        db.location_name_column,
        db.event_date_column,
      ],
      indexName: db.events_by_creator_index,
    });

    this.addGlobalSecondaryIndex({
      partitionKey: {
        name: db.sorting_key_column,
        type: dynamodb.AttributeType.STRING,
      },
      sortKey: {
        name: db.partition_key_column,
        type: dynamodb.AttributeType.STRING,
      },
      projectionType: dynamodb.ProjectionType.INCLUDE,
      nonKeyAttributes: [
        db.title_column,
        db.location_name_column,
        db.event_date_column,
      ],
      indexName: db.events_listing_index,
    });
  }

  grantQuery(principal: iam.IPrincipal) {
    this.grantReadWriteData(principal);
    principal.addToPrincipalPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.DENY,
        actions: ["dynamodb:PartiQLSelect"],
        resources: [this.tableArn],
        conditions: {
          Bool: {
            "dynamodb:FullTableScan": ["true"],
          },
        },
      }),
    );

    principal.addToPrincipalPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.DENY,
        actions: ["dynamodb:Scan"],
        resources: [this.tableArn],
      }),
    );
  }
}
