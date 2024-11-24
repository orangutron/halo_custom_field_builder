# Halo Custom Field Builder

A CLI tool for bulk creation of custom fields in Halo using a CSV file.

## Current Status

Initial development - Not yet functional

## Requirements

- Rust (latest stable version)

## Setup Guide

### API Configuration

1. Access your Halo instance at `/config/integrations/api`
2. Navigate to "View Applications"
3. Create a new application:
   - Click "New"
   - Name your application (e.g., `halo_custom_field_builder_connection`)
   - Under Details tab:
     - Select "Client ID and Secret" (OAuth2.0) authentication
     - Set Login Type as "Agent"
     - Assign an agent administrator (Recommended: Create a dedicated API service account)
   - Under Permissions tab:
     - Enable "Admin" access
   - Keep default Security tab settings
   - Save your changes

### Environment Setup

Create a `.env` file in your project root with the following configuration:

| Variable | Description | Source |
| --- | --- | --- |
| `CLIENT_ID` | OAuth2.0 client identifier | Application Details tab |
| `CLIENT_SECRET` | OAuth2.0 client secret | Generate in application edit mode* |
| `BASE_URL` | Halo instance URL | e.g.,`https://test.haloitsm.com/` |
| `TENANT` | Halo tenant name | Found on API overview page** |
| `SOURCE_FILE_NAME` | Input file name | Your CSV filename |

\* **Important**: Save the secret immediately after generation

\*\* **Note**: Leave blank for on-premise installations

#### Example `.env` Configuration

```
# Basic Halo instance info
BASE_URL=https://test.halo.com/
TENANT=test

# API Application info
CLIENT_ID=725fcb0a-91ea-4458-91dc-c4e1dc13d6a9
CLIENT_SECRET=c4cb9aea-8af7-44fe-a506-f5f0b89ca458-46cb0b44-440c-4e48-9100-f5f96bfc68be

# Source data
SOURCE_FILE_NAME=source.csv
```

> **Note**: Place your source file in the same directory as the program executable. Do not use quotes around values in the `.env` file.

## CSV Configuration

### File Format

Currently supports CSV files only (XLSX support coming soon).

### Required Fields

**name** (Required)

- System field name
- Requirements:
  - Alphanumeric characters only
  - No spaces or underscores
  - Will be prepended with "CF" by Halo

**label** (Required)

- Display name shown in forms and interfaces
- Requirements:
  - Any string
  - Cannot be a single space

**type_id** (Required)

- Field type identifier
- Requirements:
  - Must match valid type_id from Field Type Reference table below

**input_type_id** (Required)

- Input type for the field
- Requirements:
  - Must match valid input_type_id for chosen type
  - Use 0 for types with no input options

**new_values** (Optional)

- Comma-separated list of values for selection-type fields
- Requirements:
  - Required only for Single/Multiple Selection fields
  - If left blank for selection fields, field will generate with no options
  - Format: "option1,option2,option3"

### Field Type Reference

#### Basic Field Types

| Field Type | type_id | Has Input Options |
| --- | --- | --- |
| Text | 0 | Yes |
| Memo | 1 | No |
| Single Selection | 2 | Yes |
| Multiple Selection | 3 | No |
| Date | 4 | Yes |
| Time | 5 | No |
| Checkbox | 6 | No |
| Rich | 10 | No |

#### Input Options by Field Type

**Text Field Input Types** (type_id: 0)

| Input Type | input_type_id | Description |
| --- | --- | --- |
| Anything | 0 | Any text input |
| Integer | 1 | Numbers only |
| Money | 2 | Currency format |
| Alphanumeric | 3 | Letters and numbers only |
| Decimal | 4 | Numbers with decimal places |
| URL | 5 | Web address format |
| Password | 6 | Masked input field |

**Single Selection Input Types** (type_id: 2)

| Input Type | input_type_id | Description |
| --- | --- | --- |
| Standard dropdown | 0 | Basic dropdown menu |
| Tree dropdown | 1 | Hierarchical dropdown |
| Radio selection | 2 | Radio button options |

**Date Field Input Types** (type_id: 4)

| Input Type | input_type_id | Description |
| --- | --- | --- |
| Date | 0 | Date only |
| Datetime | 1 | Date and time |

**Fields with No Input Options** (always use input_type_id: 0)

- Memo (type_id: 1)
- Multiple Selection (type_id: 3)
- Time (type_id: 5)
- Checkbox (type_id: 6)
- Rich (type_id: 10)

### Example Fields

Here's a sample configuration for a pizza ordering system:

| name | label | type_id | input_type_id | new_values |
| --- | --- | --- | --- | --- |
| orderName | Order Name | 0 | 0 | |
| orderPhone | Phone Number | 0 | 1 | |
| pizzaSize | Pizza Size | 2 | 0 | Small,Medium,Large |
| crustType | Crust Type | 2 | 0 | Thin,Regular,Deep Dish,Stuffed |
| toppings | Toppings | 3 | 0 | Pepperoni,Mushrooms,Pineapple,Sausage,Green Peppers,Red Onions,Black Olives,Bacon,Ham,Ground Beef,Italian Sausage,Spinach,Fresh Tomatoes,Jalapeños,Anchovies,Chicken,Feta,Extra Mozzarella,Roasted Garlic,Artichoke Hearts |
| extraCheese | Extra Cheese | 6 | 0 | |
| specialInstructions | Special Instructions | 1 | 0 | |
| allergyNotes | Allergy Information | 10 | 0 | |
| deliveryDate | Delivery Date | 4 | 0 | |
| deliveryTime | Preferred Time | 5 | 0 | |
| paymentType | Payment Type | 2 | 2 | Cash,Card,Check |
| orderTip | Tip | 0 | 4 | |