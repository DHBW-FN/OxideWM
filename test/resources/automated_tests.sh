function setup_check() {
    state=`./target/debug/oxide-msg state`

    if grep -q "thread 'main' panicked" <<< $state; then
        echo -e "\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'oxide-msg state' could not get oxide state - Unable to run tests, aborting..."
        exit
    else
        echo -e "\x1b[32m\x1b[1mSetup Success (1/4)\x1b[0m - Can grab state from OxideWM"
    fi

    if which xterm 1>/dev/null; then
        echo -e "\x1b[32m\x1b[1mSetup Success (2/4)\x1b[0m - 'xterm' is installed."
    else
        echo -e "\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'xterm' not found - Unable to run tests, aborting..."
        exit
    fi

    if which kitty 1>/dev/null; then
        echo -e "\x1b[32m\x1b[1mSetup Success (3/4)\x1b[0m - 'kitty' is installed."
    else
        echo -e "\x1b[31m\x1b[1mCRITICAL FAILURE\x1b[0m - 'kitty' not found - Unable to run tests, aborting..."
        exit
    fi

    if ps -aux | grep "oxide-bar" 1>/dev/null; then
        echo -e "\x1b[32m\x1b[1mSetup Success (4/4)\x1b[0m - 'oxide-bar' is running."
    else
        echo -e "\x1b[33m\x1b[1mSETUP WARNING\x1b[0m - 'oxide-bar' not running - Can run tests, continuing..."
    fi
}

function run_test() {
    cmd=$1
    success_requirement=$2
    success_message=$3
    failure_message=$4
    counter=$5

    echo -ne "($counter s) Testing: '$cmd'\n\x1b[A"
    bash -c "$cmd"

    while [ $counter -gt 0 ]; do
        sleep 1
        counter=$(( $counter -1 ))
        echo -ne "($counter s) Testing: '$cmd'\n\x1b[A"
    done

    state=$(./target/debug/oxide-msg state 2>&1)
    echo $state
    if grep -q -E $success_requirement <<< $state; then
        echo -e "\x1b[32m\x1b[1mAutomatic Validation Success\x1b[0m - '$cmd' - $success_message"
    else
        echo -e "\x1b[33m\x1b[1mAutomatic Validation Failure\x1b[0m - '$cmd' - $failure_message"
    fi
}

function pause() {
    sleep 999
}

echo -e "\x1b[A\x1b[KAutomatic Validation failure might occur due to the system the tests are run on.\nPlease verify all failures manually before submitting a bug report!\n"

echo -e "Setting up tests..."
sleep 5 # Sleep required as oxide needs a few seconds to set up it's ipc channel
setup_check

echo -e "\nTesting..."

# Command - Success requirement - Success message - Failure message - Sleep duration before status request
oxidemsg=./target/debug/oxide-msg
run_test "$oxidemsg exec kitty" "(kitty.*){1}" "Successfully opened a window" "Failed to open a window" 1
run_test "$oxidemsg exec kitty" "(kitty.*){2}" "Successfully opened a window" "Failed to open a window" 1
run_test "$oxidemsg exec kitty" "(kitty.*){3}" "Successfully opened a window" "Failed to open a window" 1
run_test "$oxidemsg exec kitty" "(kitty.*){4}" "Successfully opened a window" "Failed to open a window" 1
run_test "$oxidemsg exec xterm" "(xterm.*){1}" "Successfully opened a window" "Failed to open a window" 7


run_test "$oxidemsg focus left" "kitty.*xterm" "Moved focus left" "Failed to move focus left" 1
run_test "$oxidemsg focus left" "kitty.*xterm" "Moved focus left" "Failed to move focus left" 1
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){1}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){2}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){3}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){4}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){3}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){2}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){1}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){0}" "Moved window right" "Failed to move window right" 2

run_test "$oxidemsg layout horizontal" "[hH]orizontal" "Successfully set layout to 'HorizontalStriped'" "Failed to set layout to 'HorizontalStriped'" 1
# run_test "$oxidemsg focus left" "kitty.*xterm" "Moved focus left" "Failed to move focus left" 1
# run_test "$oxidemsg focus left" "kitty.*xterm" "Moved focus left" "Failed to move focus left" 1
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){1}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){2}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){3}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){4}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){3}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){2}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){1}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){0}" "Moved window right" "Failed to move window right" 2

run_test "$oxidemsg layout vertical" "[vV]iled" "Successfully set layout to 'Vertical'" "Failed to set layout to 'Vertical'" 1
# run_test "$oxidemsg focus left" "kitty.*xterm" "Moved focus left" "Failed to move focus left" 1
# run_test "$oxidemsg focus left" "kitty.*xterm" "Moved focus left" "Failed to move focus left" 1
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){1}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){2}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){3}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move left" ".*xterm.*(.*kitty.*){4}" "Moved window left" "Failed to move window left" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){3}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){2}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){1}" "Moved window right" "Failed to move window right" 2
run_test "$oxidemsg move right" ".*xterm.*(.*kitty.*){0}" "Moved window right" "Failed to move window right" 2

run_test "$oxidemsg layout tiled" "[tT]iled" "Successfully set layout to 'Tiled'" "Failed to set layout to 'Tiled'" 1
run_test "$oxidemsg kill" "(xterm){0}" "Successfully closed a window" "Failed to close a window" 2
run_test "$oxidemsg quit" "MethodError" "Successfully quit oxide" "Failed to quit oxide" 2

exit
