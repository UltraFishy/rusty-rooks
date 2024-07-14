import subprocess
import time

def scale_clients(desired_replicas):
    subprocess.run(["docker", "service", "scale", f"rusty={desired_replicas}"])

def monitor_connections():
    active_connections = 0
    while True:
        # Check the number of active connections
        result = subprocess.run(["netstat", "-an"], stdout=subprocess.PIPE)
        output = result.stdout.decode('utf-8')
        new_connections = output.count(':4321')  # Ensure to count connections on the correct port

        if new_connections != active_connections:
            active_connections = new_connections
            scale_clients(active_connections)
            print(f"Has {active_connections} connections")

        time.sleep(5)

if __name__ == "__main__":
    monitor_connections()
