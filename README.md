# Secure File Shredder System (SFS)

SFS is a secure file shredder system written in Rust that allows users to securely delete files from a designated directory. It provides two user levels: employee and admin, each with specific permissions and functionalities. The system ensures data security by implementing standard authentication procedures and logging relevant activities.

## Features

1. **User Authentication**: SFS requires users to authenticate themselves before accessing the system. It supports standard authentication methods to ensure secure access.

2. **User Levels**:
   - *Employee*: Employees can search for files and submit deletion requests. However, they cannot delete files directly.
   - *Admin*: Admins have the authority to approve deletion requests submitted by employees. Additionally, they can perform all actions available to employees.

3. **File Search**: Users can search for files within the designated directory to locate the files they want to delete.

4. **Deletion Requests**: Employees can submit deletion requests for files they wish to delete. These requests require approval from an admin before the files are permanently deleted.

5. **Customization of User Details**: Users can customize their details, including username, password, and other relevant information.

6. **Logging**: SFS logs all relevant activities, including login attempts, file searches, deletion requests, and approval actions. This ensures accountability and traceability of user actions within the system.

## Installation

To install SFS, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/nafdev01/file_shredder.git
   ```

2. Navigate to the project directory:
   ```
   cd file_shredder
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. Run the application:
   ```
   ./target/release/file_shredder
   ```

## Usage

Upon running the application, users will be prompted to log in using their credentials. Depending on their user level, they will have access to different functionalities:

- **Employee**:
  - Search for files.
  - Submit deletion requests.
  - Customize user details.

- **Admin** (in addition to employee functionalities):
  - Approve deletion requests.

## Configuration

SFS allows for the configuration of various parameters, including:
- Directory path: Specify the directory where files are stored for deletion.
- User authentication: Configure the authentication method and requirements.
- Logging settings: Define the log format and destination.

These configurations can be adjusted in the `config.toml` file located in the project directory.

## Contributing

Contributions to SFS are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure they pass all tests.
4. Commit your changes with descriptive commit messages.
5. Push your changes to your fork.
6. Open a pull request to the main repository.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Support

If you encounter any issues or have questions about SFS, feel free to [open an issue](https://github.com/yourusername/sfs/issues) on GitHub or contact the project maintainer directly.

## Acknowledgments

SFS was inspired by the need for a secure and reliable file shredder system for businesses and organizations. We extend our gratitude to the Rust community for their support and contributions to the ecosystem.

Thank you for using SFS! We hope it helps you securely manage your files with ease.
