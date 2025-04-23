# Programming in Rust

This repository contains a collection of Rust programming exercises and projects to demonstrate various Rust concepts and techniques.

## Projects

### 1. Calculate Roots (`/01_Calculate_roots`)
A program to calculate the roots of a quadratic equation using the delta method.

Features:
- Calculates the discriminant (delta) of a quadratic equation
- Computes the roots when delta is non-negative
- Includes unit tests to verify functionality

### 2. Temperature Converter (`/03_tmp_converter`)
Utility for converting temperatures with error handling.

Features:
- Parses temperature from optional string input
- Reads temperature from file
- Implements robust error handling

### 3. Account Status Checker (`/04_Account_status_checker`)
Demonstrates user authentication and status verification.

Features:
- User structure with username and active status
- Function to locate active users by username
- Error handling for missing or inactive users

### 4. Parsing Numbers (`/05_Parsing_nums`)
Utility for parsing a collection of optional string values into integers.

Features:
- Converts `Vec<Option<&str>>` to valid integers
- Collects and separates successful parses and errors
- Demonstrates use of iterators and error handling

### 5. Command Interpreter (`/06_Command_interpreter`)
A simple command-line calculator that interprets and executes basic arithmetic operations.

Features:
- Supports addition, subtraction, multiplication, and division
- Validates input and handles errors
- Demonstrates pattern matching and result handling

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.