#include <iostream>
#include <string>
#include <grpcpp/grpcpp.h>
#include <Talk.grpc.pb.h>
#include <ctime> 

using grpc::Status;
using grpc::Channel;
using grpc::ClientContext;
using grpc::ClientWriter;
using grpc::ServerContext;
using Talk::HelloReply;
using Talk::HelloRequest;
using Talk::Greeter;

class GreeterClient
{
public:
	GreeterClient(std::shared_ptr<Channel> channel) : serverStub(Greeter::NewStub(channel)) {}
	void SendMesg(std::string name, std::string time)
	{
		HelloRequest request;
		HelloReply reply;
		Status status;
		ClientContext context;
		request.set_date(time);
		request.set_name(name);
		status = serverStub->SayHello(&context, request, &reply);
		if (status.ok())
		{
			std::cout << reply.message() << std::endl;
		}
		else
		{
			std::cout << "\r";
			std::cout << status.error_code() << ": " << status.error_message() << std::endl;
		}
	}
private:
	std::unique_ptr<Greeter::Stub> serverStub;
};

int main()
{
	grpc_init();
	std::string name;
	std::string time;
	std::time_t t;	
	std::tm* now = new std::tm;	
	std::cout << "Enter your name: ";
	std::cin >> name;
	t = std::time(0);
	localtime_s(now, &t);
	time = std::to_string(now->tm_hour) + ":" + std::to_string(now->tm_min) + ":" + std::to_string(now->tm_sec);
	GreeterClient client(grpc::CreateChannel("127.0.0.1:50051", grpc::InsecureChannelCredentials()));
	client.SendMesg(name, time);
	system("pause");
	grpc_shutdown();
	return 0;
}