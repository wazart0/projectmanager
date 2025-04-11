import pandas as pd
import json
from datetime import datetime, timedelta
from dateutil.relativedelta import relativedelta


APP_PATH = '/app'


project_info = json.load(open(f"{APP_PATH}/local/tmp/project_info.json"))

print(project_info)


diff = relativedelta(datetime.fromisoformat(project_info['project_finish']), datetime.fromisoformat(project_info['project_start']))

print(diff.months)



