import logging

def setup_logging():
    logging.basicConfig(level=logging.INFO,
                        datefmt="%m-%d %H:%M",
                        filename="output.log",
                        filemode="w")

    console = logging.StreamHandler()
    console.setLevel(logging.INFO)

    formatter = logging.Formatter("%(name)-12s: %(levelname)-8s %(message)s")

    console.setFormatter(formatter)

    logging.getLogger("").addHandler(console)

    logging.getLogger("app.logger").info("Logger has been initialized")

