.. _dbus:

======================================
D-Bus interprocess communication (IPC)
======================================

D-Bus interface description
---------------------------

OxideWM has a D-Bus interface for IPC communication. This is primarily used in the ``Oxide-IPC`` library.
This interface mainly gives access to the current state of *Oxide*. This state includes the loaded config, current windows, layouts, workspaces...
It also allows to execute oxide commands.

Interface
^^^^^^^^^

.. code-block:: bash

    org.oxide.interface


D-Bus Method Calls
^^^^^^^^^^^^^^^^^^

Returns the current `OxideState` as a JSON object:

.. code-block:: bash

    get_state() -> String


Executes the given command:

.. code-block:: bash

    sent_event(WmActionEvent) -> void


D-Bus Signal
^^^^^^^^^^^^

Returns the current oxide state when change occurs to the subscribers:

.. code-block:: bash
    
    state_change -> String

