
def getLines(filename):
  with open(filename, 'r') as fp:
    return [l.strip('\n') for l in fp.readlines()]

