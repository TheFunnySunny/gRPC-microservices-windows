#include <iostream>
#include <memory>
#include <string>
#include <grpcpp/grpcpp.h>
#include <Talk.grpc.pb.h>

using grpc::Status;
using grpc::Channel;
using grpc::Server;
using grpc::Service;
using grpc::ServerContext;
using grpc::ServerBuilder;
using Talk::HelloReply;
using Talk::HelloRequest;
using Talk::Greeter;

using namespace Talk;

class TalkingService final : public  Greeter::Service
{
	Status SayHello(ServerContext* context, const HelloRequest* cText, HelloReply* sText) override
	{
		std::cout << "Hello " << cText->name()<<std::endl;
		std::cout << "Your time is: " << cText->date()<<std::endl;
		sText->set_message("Hello " + cText->name());
		return Status::OK;
	}
};

void RunServer()
{
	std::cout << "Server stated listening on port 50051." << std::endl;
	std::string ServerAddr("0.0.0.0:50051");
	TalkingService msgTalkService;
	ServerBuilder builder;
	builder.AddListeningPort(ServerAddr, grpc::InsecureServerCredentials());
	builder.RegisterService(&msgTalkService);
	std::unique_ptr<Server> server(builder.BuildAndStart());
	server->Wait();
}

int main()
{
	RunServer();
	return 0;
}