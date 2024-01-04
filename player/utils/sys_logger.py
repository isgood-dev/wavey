import logging
import sys

def setup_logging():
    logging.basicConfig(level=logging.DEBUG,
                        datefmt="%m-%d %H:%M",
                        filename="output.log",
                        filemode="w")

    formatter = logging.Formatter("%(name)-12s: %(levelname)-8s %(message)s")

    console = logging.StreamHandler(sys.stdout)
    console.setLevel(logging.DEBUG)
    console.setFormatter(formatter)

    logging.getLogger("app.stdout").addHandler(console)
    logging.getLogger("app.logger").info("Logger has been initialized")

