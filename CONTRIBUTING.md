# Contributing

Contributions to **WarpZip** are welcome! To contribute:

1. Fork the repository.

2. Create a new branch (`git checkout -b feature-branch`).

3. Make your changes and commit them. Please use the following commit message format to maintain consistency:
   
   ## Predefined Commit Types:
   
   - 📝 Update release notes
   - ✏️ Typo corrections
   - ⬆️ Major updates or new releases
   - 🚚 Redirection/rename changes
   - 📄 Add licenses, markdowns, etc.
   - 🔨 Bug fixes
   - ✨ New features or enhancements
   - 🔧 Technical improvements or general adjustments
   - 🗑️ Remove unused code or files
   - 📚 Update documentation
   - ♻️ Use of best practices and code reuse
   - 👷 Troubleshooting issues
   - 🎨 Code refactoring (without changing functionality)
   - 🚀 Performance improvements
   
   ## Example Commit Message:
   
   ✨ feature: Added support for `.tar.xz` format
   
   Please choose one of the types and prefix your commit message with the corresponding emoji, followed by a detailed description.

4. Push to your branch (`git push origin feature-branch`).

5. Open a pull request describing your changes.

Please ensure that your code is well-documented, follows Rust's formatting standards (run `cargo fmt`), and includes relevant tests where applicable.

## Code of Conduct for Pull Requests

To ensure a smooth and effective collaboration process, please follow these guidelines when submitting pull requests:

1. **Limit the Scope**: Ensure that each pull request (PR) addresses one task or issue at a time. Avoid bundling multiple unrelated changes into a single PR. This makes it easier to review and understand the changes.

2. **Avoid Unnecessary Modifications**: Do not modify parts of the code that are not directly related to the issue or feature you are working on. For example, changing comments, formatting, or code unrelated to the task makes the PR harder to review and increases the chances of introducing errors.
   
   **Example of an unnecessary change**:
   
   - Changing a comment for a function that you are not modifying (e.g., correcting a comment about a function while the function itself remains unchanged).

3. **Provide Clear Commit Messages**: As described in the previous section, always follow the predefined commit message format to keep the history of the project clear and understandable.

4. **Write Meaningful Descriptions**: When opening a pull request, ensure that the description clearly explains the purpose of the PR. Link it to the issue it addresses, if applicable, and provide context where needed. This helps maintainers understand the intent behind the changes.

5. **Test Your Changes**: Before submitting a pull request, make sure that your changes work as expected and do not break existing functionality. Run the tests and check for any regressions. If you introduce new functionality, ensure you add tests for it.

6. **Keep Changes Minimal and Reversible**: Ensure that each change can be easily reversed if necessary. Avoid changes that may be difficult to backtrack, especially if they involve large-scale refactoring or complex rewrites.

7. **Respect the Project’s Style**: Please review the [Project Style Guide](project-style.md) to follow the existing coding style of the project, including naming conventions, indentation, and structure. If you're unsure about anything, refer to this document or ask for clarification.

8. **Review Before Submitting**: Take the time to review your PR before submitting it. Ensure that there are no extraneous changes or typos, and that the code is clear and understandable.

9. **Be Open to Feedback**: When reviewers leave comments or suggestions, take the time to understand them and make the necessary adjustments. Open collaboration and constructive criticism are key to improving the project.