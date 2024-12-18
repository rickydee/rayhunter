from pycrate_mobile.NAS import parse_NAS_MO
from sys import stdin
import base64

buf = base64.b64decode(stdin.read())
print(parse_NAS_MO(buf))
