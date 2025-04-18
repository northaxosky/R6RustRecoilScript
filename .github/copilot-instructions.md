<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Instructions for Copilot

This project is a Rust-based recoil control script for Rainbow Six Siege. The following guidelines should be followed when generating code for this workspace:

1. **Recoil Control Logic**:
   - The script should simulate mouse movement to counteract recoil when the left mouse button is held down.
   - The logic should mimic the Lua script provided earlier, where recoil control is active only when both the left and right mouse buttons are pressed.
   - Use the `rdev` crate for detecting mouse events and the `enigo` crate for simulating mouse movement.

2. **GUI Integration**:
   - The GUI is built using the `eframe` crate.
   - The GUI should include buttons to toggle the recoil control script on and off.
   - Ensure the GUI and recoil control logic run concurrently without blocking each other.

3. **Error Handling**:
   - Handle errors gracefully, especially when dealing with mouse event listeners.
   - Log meaningful error messages to the console for debugging purposes.

4. **Performance**:
   - Avoid busy-waiting loops; use appropriate delays to reduce CPU usage.
   - Ensure the script is responsive to user input, such as stopping recoil control immediately when buttons are released.

5. **Platform-Specific Considerations**:
   - This project is being developed on Windows. Ensure compatibility with the Windows platform.

6. **Code Quality**:
   - Follow Rust best practices for code readability and maintainability.
   - Use meaningful variable names and add comments where necessary to explain complex logic.

7. **Testing**:
   - Test the script thoroughly to ensure it behaves as expected under different scenarios.
   - Verify that the GUI and recoil control logic work seamlessly together.

Feel free to update these instructions as the project evolves.