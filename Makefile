NAME = kay/freeze-serve-http-server
TAG = v1
INSTANCE = freeze-serve-http-server-app

all =  default build run buildrun details clean

.PHONY: all

default: buildrun

build:
	docker-compose build 

run:
	docker-compose up

clean:
	docker-compose down

details:
	@echo "Name: "  $(NAME) "\nTag: " $(TAG)  "\nInstance: " $(INSTANCE) 

buildrun: details build run


