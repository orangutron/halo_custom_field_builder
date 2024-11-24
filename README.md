# Halo Custom Field Builder

A CLI tool for bulk creation of custom fields in Halo.

## Current Status

Initial development - Not yet functional

## Requirements

- Rust (latest stable version)

## Configuration

### Setting up API Access

1. Navigate to `/config/integrations/api` in your Halo instance
2. Go to "View Applications"
3. Click "New" to create an application
4. Configure the application:
   - Give it a meaningful name (e.g., `halo_custom_field_builder_connection`)
   - In Details tab, select "Client ID and Secret" (OAuth2.0) authentication
   - Set Login Type as "Agent"
   - Select an agent administrator (Best practice: Create a dedicated API service account)
   - Under Permissions tab, enable "Admin" access
   - Leave Security tab at default settings
   - Save the application

### Environment Variables

Create a `.env` file in the project root with the following variables:


| Variable         | Description                | How to Obtain                                                            |
| ------------------ | ---------------------------- | -------------------------------------------------------------------------- |
| CLIENT_ID        | OAuth2.0 client identifier | Found in the Details tab of your application                             |
| CLIENT_SECRET    | OAuth2.0 client secret     | Generate in application's edit mode.**Important**: Save after generating |
| BASE_URL         | Your Halo instance URL     | Example:`https://test.haloitsm.com/`                                     |
| TENANT           | Your Halo tenant name      | Found on API overview page. Leave blank for on-premise installations     |
| SOURCE_FILE_NAME | Name of input file         | Name of your .csv or .xlsx file (include extension)                      |

### Example .env file:
BASE_URL=https://test.halo.com/
TENANT=test

CLIENT_ID=725fcb0a-91ea-4458-91dc-c4e1dc13d6a9
CLIENT_SECRET=c4cb9aea-8af7-44fe-a506-f5f0b89ca458-46cb0b44-440c-4e48-9100-f5f96bfc68be

SOURCE_FILE_NAME=source.xlsx

**Note**: Place your source file in the same directory as the program executable.

**Note for .env files**: When filling in the .env file, do not use quotes around the values - enter them in the exact format as shown in the example above.