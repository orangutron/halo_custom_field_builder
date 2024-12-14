# Halo Custom Field Builder

A CLI tool for bulk creation of custom fields in Halo using a CSV file.

## Current Status

- ✅ Environment configuration validation
- ✅ CSV input validation with detailed error messages
- ✅ JSON transformation with Halo API format
- ✅ API integration with authentication
- ✅ Debug mode for field-by-field processing
- ✅ Comprehensive logging system
- ✅ Error handling and reporting

## Features

- Validates environment configuration (URLs, credentials, file paths)
- Validates CSV field definitions against Halo's requirements
- Transforms validated fields to Halo API JSON format
- Provides clear error messages for configuration and data issues
- Supports OAuth2.0 authentication with Halo API
- Includes debug mode for careful field review
- Maintains detailed operation logs with automatic rotation
- Offers both bulk import and field-by-field processing

## About

A CLI tool for bulk creation of custom fields in Halo using a CSV file. Built with Rust.

## Requirements

- Valid `.env` configuration file
- Properly formatted CSV input file

That's it! The program is distributed as a standalone executable, so no additional runtime dependencies are required.

## Setup Guide

### Environment Setup

Create a `.env` file in your project root with the following configuration:


| Variable           | Required | Description                | Format Requirements                                                                           |
| -------------------- | ---------- | ---------------------------- | ----------------------------------------------------------------------------------------------- |
| `BASE_URL`         | Yes      | Halo instance URL          | Must start with 'https://' and contain only the base domain (e.g., 'https://test.halo.com'). Do not include paths like '/api' or '/auth' |
| `TENANT`           | No       | Halo tenant name           | Can be empty for on-premise installations                                                     |
| `CLIENT_ID`        | Yes      | OAuth2.0 client identifier | Cannot be empty                                                                               |
| `CLIENT_SECRET`    | Yes      | OAuth2.0 client secret     | Cannot be empty                                                                               |
| `SOURCE_FILE_NAME` | Yes      | Input file name            | Cannot be empty                                                                               |

#### Example `.env` Configuration

```env
# Basic Halo instance info
BASE_URL=https://test.halo.com
TENANT=test

# API Application info
CLIENT_ID=dd5ef51d-ec0f-4247-b79d-1234b0e40dec
CLIENT_SECRET=8595ec7e-81e5-4a17-1234-6c3ae166e0c7-f65cde17-37da-4cf1-89de-1fdda60d915b

# Source data
SOURCE_FILE_NAME=source.csv
```

> **Important Notes**:
>
> - The `.env` file must exist in the project root
> - All fields except TENANT must have non-empty values
> - URLs will be automatically normalized to use HTTPS and remove trailing slashes
> - Do not use quotes around values in the `.env` file

## CSV Configuration

### Required Columns

The CSV file must include all of these columns with exact names:

- `name`
- `label`
- `type_id`
- `input_type_id`
- `options`

### Field Validation Rules

**name** (Required)

- Must not be empty
- Must contain only alphanumeric characters
- Spaces and special characters are not allowed

**label** (Required)

- Must not be empty
- Cannot be a single space
- Can contain any visible characters

**type_id** (Required)

- Must be a valid numeric value from the Field Type Reference table
- Must match one of: 0, 1, 2, 3, 4, 5, 6, 10

**input_type_id** (Required)

- Must be valid for the selected type_id (see Input Options by Field Type)
- Validation varies by field type:
  - Text (0): Values 0-6 allowed
  - Single Selection (2): Values 0-2 allowed
  - Date (4): Values 0-1 allowed
  - Others: Must be 0

**options** (Conditional)

- Required for Single/Multiple Selection fields (type_id: 2 or 3)
- Must not be empty for selection fields
- Format as comma-separated values
- Optional for all other field types

### Field Type Reference

#### Basic Field Types


| Field Type         | type_id | Has Input Options |
| -------------------- | --------- | ------------------- |
| Text               | 0       | Yes               |
| Memo               | 1       | No                |
| Single Selection   | 2       | Yes               |
| Multiple Selection | 3       | No                |
| Date               | 4       | Yes               |
| Time               | 5       | No                |
| Checkbox           | 6       | No                |
| Rich               | 10      | No                |

#### Input Options by Field Type

**Text Field Input Types** (type_id: 0)


| Input Type   | input_type_id | Description                 |
| -------------- | --------------- | ----------------------------- |
| Anything     | 0             | Any text input              |
| Integer      | 1             | Numbers only                |
| Money        | 2             | Currency format             |
| Alphanumeric | 3             | Letters and numbers only    |
| Decimal      | 4             | Numbers with decimal places |
| URL          | 5             | Web address format          |
| Password     | 6             | Masked input field          |

**Single Selection Input Types** (type_id: 2)


| Input Type        | input_type_id | Description           |
| ------------------- | --------------- | ----------------------- |
| Standard dropdown | 0             | Basic dropdown menu   |
| Tree dropdown     | 1             | Hierarchical dropdown |
| Radio selection   | 2             | Radio button options  |

**Date Field Input Types** (type_id: 4)


| Input Type | input_type_id | Description   |
| ------------ | --------------- | --------------- |
| Date       | 0             | Date only     |
| Datetime   | 1             | Date and time |

**Fields with No Input Options** (always use input_type_id: 0)

- Memo (type_id: 1)
- Multiple Selection (type_id: 3)
- Time (type_id: 5)
- Checkbox (type_id: 6)
- Rich (type_id: 10)

### Example Fields

Here's a sample configuration for a pizza ordering system:


| name                | label                | type_id | input_type_id | options                                                                                                                                                                                                                     |
| --------------------- | ---------------------- | --------- | --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| orderName           | Order Name           | 0       | 0             |                                                                                                                                                                                                                             |
| orderPhone          | Phone Number         | 0       | 1             |                                                                                                                                                                                                                             |
| pizzaSize           | Pizza Size           | 2       | 0             | Small,Medium,Large                                                                                                                                                                                                          |
| crustType           | Crust Type           | 2       | 0             | Thin,Regular,Deep Dish,Stuffed                                                                                                                                                                                              |
| toppings            | Toppings             | 3       | 0             | Pepperoni,Mushrooms,Pineapple,Sausage,Green Peppers,Red Onions,Black Olives,Bacon,Ham,Ground Beef,Italian Sausage,Spinach,Fresh Tomatoes,Jalapeños,Anchovies,Chicken,Feta,Extra Mozzarella,Roasted Garlic,Artichoke Hearts |
| extraCheese         | Extra Cheese         | 6       | 0             |                                                                                                                                                                                                                             |
| specialInstructions | Special Instructions | 1       | 0             |                                                                                                                                                                                                                             |
| allergyNotes        | Allergy Information  | 10      | 0             |                                                                                                                                                                                                                             |
| deliveryDate        | Delivery Date        | 4       | 0             |                                                                                                                                                                                                                             |
| deliveryTime        | Preferred Time       | 5       | 0             |                                                                                                                                                                                                                             |
| paymentType         | Payment Type         | 2       | 2             | Cash,Card,Check                                                                                                                                                                                                             |
| orderTip            | Tip                  | 0       | 4             |                                                                                                                                                                                                                             |

## Known Limitations

- The program currently only supports field creation (not updating or deleting)
- All fields are created with default usage and searchable settings
- Rate limiting is implemented to respect API constraints:
  - API limit: 700 requests per 5-minute rolling window
  - Program enforces 500ms delay between requests (~120 requests/minute)
  - This ensures staying well under the API rate limit while maintaining reliability
- Batch processing is limited to one field at a time to ensure proper error handling

## Rate Limiting

### API Constraints
The Halo API implements rate limiting of 700 requests per 5-minute rolling window. To ensure reliable operation and prevent throttling, this program implements a conservative rate limiting strategy:

- Enforces a 500ms delay between each field creation request
- Results in approximately 120 requests per minute
- Stays well under the API limit of 700 requests per 5 minutes
- No manual throttling required from the user

### Impact on Processing Time
Due to the rate limiting and API processing time:
- Each field takes approximately 1 second to process (500ms enforced delay + API response time)
- 100 fields ≈ 2 minutes
- 500 fields ≈ 10 minutes
- 1000 fields ≈ 17 minutes (based on actual testing)

Real-world testing with 1000 fields completed in approximately 17 minutes (15:10:34 to 15:27:05), which accounts for:
- The 500ms enforced delay between requests
- Halo API processing time
- Network latency
- Response handling

This controlled pacing helps ensure:
- Reliable field creation
- No API throttling errors
- Predictable processing times
- Minimal impact on API performance

## Error Handling

The program includes comprehensive error handling for:
- Environment configuration issues
- CSV file validation
- API authentication
- Field creation failures

Each error provides specific details about:
- The location of the error (row number for CSV errors)
- The nature of the problem
- Suggested fixes where applicable

## Logging

The program maintains detailed logs of all operations:
- Logs are stored in the `logs` directory
- Log files are automatically rotated (keeping last 7 days)
- Maximum of 100 log files are retained
- Each log includes:
  - Timestamp
  - Operation type
  - Success/failure status
  - Detailed error messages when applicable

## Debug Mode

The program includes a debug mode that allows you to:
- Process fields one at a time
- Review field details before processing
- Skip specific fields
- Get immediate feedback on success/failure
- Exit at any point

## Distributable Structure

The program distribution includes the following files:

| File/Folder                | Purpose                                          | Notes                                            |
|---------------------------|--------------------------------------------------|--------------------------------------------------|
| `halo_custom_field_builder.exe` | Main executable                            | Core program                                     |
| `.env`                    | Configuration file                               | Must be named exactly ".env" with required variables |
| `source.csv`              | Your input CSV file                              | Must match the name specified in .env            |
| `logs/`                   | Directory for log files                          | Created automatically on first run               |

### Important Requirements

| Requirement              | Description                                                                |
|-------------------------|----------------------------------------------------------------------------|
| `.env` Location         | Must be in the same directory as the executable                            |
| Source File Location    | Must be in the same directory as the executable                            |
| Program Launch          | Simply double-click the executable to run the program                      |
| Logs Directory          | Created automatically on first run in the executable directory             |