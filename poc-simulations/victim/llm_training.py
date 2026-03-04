import torch
from transformers import GPT2LMHeadModel, GPT2Tokenizer
import time
import os

print("--- SYSTEM: Starting Corporate AI Training Pipeline ---")

# 1. LOAD THE BASE MODEL
# We are downloading the "brain" (model) and the "dictionary" (tokenizer) of GPT-2.
print("Loading small LLM (GPT-2)...")
tokenizer = GPT2Tokenizer.from_pretrained("gpt2")
model = GPT2LMHeadModel.from_pretrained("gpt2")

# We create some fake "proprietary company data" to train the model on.
# The tokenizer converts this text into numbers the AI can understand.
dummy_data = "Latent-Sentry is a highly confidential kernel security project."
inputs = tokenizer(dummy_data, return_tensors="pt")

# 2. THE TRAINING LOOP (Generating the Hardware Workload)
# This loop simulates the heavy "thinking" phase. It forces your CPU/GPU to work hard.
# Your eBPF sensors will see this as legitimate, authorized hardware usage.
print("\nStarting Fine-Tuning Process...")
model.train() # Put the model into training mode

for epoch in range(1, 4):
    print(f"  -> Training Epoch {epoch}/3... (Simulating heavy matrix multiplication)")
    
    # Forward pass: The AI tries to guess the next word
    outputs = model(**inputs, labels=inputs["input_ids"])
    loss = outputs.loss
    
    # Backward pass: The AI corrects its mistakes (This takes the most compute power)
    loss.backward()
    
    # Sleep just to slow it down so the professors can watch it happen
    time.sleep(2) 

print("\n--- SYSTEM: Training Complete! ---")

# 3. SAVING THE MODEL (The Moment of Vulnerability)
# This is the most critical part for your project. We are taking the trained model
# out of the RAM and writing it to the hard drive. 
save_folder = "./secret_company_weights"
os.makedirs(save_folder, exist_ok=True)

print(f"Saving proprietary model weights to disk at: {save_folder}")
model.save_pretrained(save_folder)
tokenizer.save_pretrained(save_folder)

print("--- SYSTEM: Model safely stored. AI Pipeline shutting down. ---")
