# Halo Custom Field Builder

A CLI tool for bulk creation of custom fields in Halo using a CSV file.

## Current Status

Initial development - Input validation implemented

## Requirements

- Rust (latest stable version)
- Valid `.env` configuration file
- Properly formatted CSV input file

## Setup Guide

### Environment Setup

Create a `.env` file in your project root with the following configuration:


| Variable           | Required | Description                | Format Requirements                                                                           |
| -------------------- | ---------- | ---------------------------- | ----------------------------------------------------------------------------------------------- |
| `BASE_URL`         | Yes      | Halo instance URL          | Must be a valid HTTPS URL. Will be normalized to include 'https://' and remove trailing slash |
| `TENANT`           | No       | Halo tenant name           | Can be empty for on-premise installations                                                     |
| `CLIENT_ID`        | Yes      | OAuth2.0 client identifier | Cannot be empty                                                                               |
| `CLIENT_SECRET`    | Yes      | OAuth2.0 client secret     | Cannot be empty                                                                               |
| `SOURCE_FILE_NAME` | Yes      | Input file name            | Cannot be empty                                                                               |

#### Example `.env` Configuration

```env
# Basic Halo instance info
BASE_URL=https://customfieldbuildertest1.haloitsm.com
TENANT=customfieldbuildertest1

# API Application info
CLIENT_ID=dd5ef51d-ec0f-4247-b79d-4152b0e40dec
CLIENT_SECRET=8595ec7e-81e5-4a17-b5cf-6c3ae166e0c7-f65cde17-37da-4cf1-89de-1fdda60d915b

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
