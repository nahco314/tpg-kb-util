import subprocess


st = subprocess.Popen("cargo run -r", stdout=subprocess.PIPE).stdout

while True:
    p_id, value = st.read(2)
    keyboard_id = p_id >> 4
    type_id = p_id % 16

    print(f"{type_id} on {keyboard_id} : {value}")
