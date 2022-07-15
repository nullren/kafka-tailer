kafka-tailer-linux:
	docker build -t kafka-tailer-linux .
	docker container create --name kafka-tailer-linux kafka-tailer-linux
	docker container cp kafka-tailer-linux:/usr/local/bin/kafka-tailer kafka-tailer-linux
	docker container rm kafka-tailer-linux
