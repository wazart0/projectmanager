import os
from dotenv import dotenv_values

# Load environment variables from .env file
env_path = os.path.join(os.path.dirname(__file__), '.env')

CONFIG = dotenv_values(env_path)

# Override loaded values with environment variables if they exist
CONFIG.update({
    key: os.environ.get(key, value)
    for key, value in CONFIG.items()
})


print(CONFIG)

