import os
import time
import shutil

print(" [!] ROOTKIT ACTIVE: Shadow Process Started.")
print(" [!] Monitoring system for high-value AI weights...")

# This is the folder your LLM script creates
target_directory = "./secret_company_weights"

# The hacker script loops endlessly, checking the hard drive every 1 second
while True:
    # If the folder exists AND it has files inside it (meaning the LLM saved successfully)
    if os.path.exists(target_directory) and len(os.listdir(target_directory)) > 0:
        print(f"\n [!!!] TARGET ACQUIRED: Proprietary AI weights detected in {target_directory}!")
        print(" [!!!] Executing massive sys_read to steal the files...")
        
        # Simulate the theft by zipping the folder
        stolen_file_name = "stolen_ai_model"
        shutil.make_archive(stolen_file_name, 'zip', target_directory)
        
        print(f" [!!!] EXFILTRATION SUCCESS: Weights compressed to {stolen_file_name}.zip")
        print(" [!!!] Uploading to remote hacker server over TCP socket...")
        print(" [!] ASSET COMPROMISED. SHUTTING DOWN SHADOW PROCESS.")
        break
        
    time.sleep(1) # Wait 1 second before checking again
