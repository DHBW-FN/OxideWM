cargo build --example window_with_frame

XEPHYR=$(whereis -b Xephyr | cut -f2 -d ' ')
xinit ./xinitrc -- $XEPHYR :100 -ac -screen 500x500 -host-cursor 
