import dbus

# 1. Configuration
BUS_NAME = "org.freedesktop.impl.portal.desktop.rust_backend"
OBJECT_PATH = "/org/freedesktop/portal/desktop"
INTERFACE = "org.freedesktop.impl.portal.FileChooser"


def test_filechooser():
    try:
        session_bus = dbus.SessionBus()
        proxy = session_bus.get_object(BUS_NAME, OBJECT_PATH)
        interface = dbus.Interface(proxy, INTERFACE)

        print(f"Calling OpenFile on {BUS_NAME}...")

        # --- 2. Prepare Arguments ---

        # Arg 1: handle (Object Path)
        # The frontend usually generates this. We fake it here.
        handle = dbus.ObjectPath("/org/freedesktop/portal/desktop/request/sender/token")

        # Arg 2: app_id (String)
        # The ID of the app "requesting" the file.
        app_id = "org.example.TestScript"

        # Arg 3: parent_window (String)
        parent_window = ""

        # Arg 4: title (String)
        title = "Test Rust Backend Direct"

        # Arg 5: options (Dictionary a{sv})
        options = dbus.Dictionary(
            {
                "multiple": False,
                "directory": False,
                "modal": True,
                # Add filters if your backend supports them
            },
            signature="sv",
        )

        # --- 3. Call the method ---
        # Your Rust signature returns (u32, HashMap<String, OwnedValue>)
        # This maps to D-Bus (u, a{sv})
        response_code, results = interface.OpenFile(
            handle, app_id, parent_window, title, options
        )

        # --- 4. Handle Results ---
        print("\n--- Method Returned ---")

        # Response codes: 0 = Success, 1 = Cancel, 2 = Other
        if response_code == 0:
            print("Status: Success (0)")
            uris = results.get("uris", [])
            print(f"Selected URIs: {uris}")
        elif response_code == 1:
            print("Status: User Cancelled (1)")
        else:
            print(f"Status: Other ({response_code})")

        print(f"Full Result Dictionary: {results}")

    except dbus.exceptions.DBusException as e:
        print(f"D-Bus Error: {e}")


if __name__ == "__main__":
    test_filechooser()
