using Grpc.Core;
using Talk;
using System.Threading.Tasks;
using System;
namespace MyServer
{
    class TalkImpl : Greeter.GreeterBase
    {
        // Server side handler of the SayHello RPC
        public override Task<HelloReply> SayHello(HelloRequest request, ServerCallContext context)
        {
            Console.WriteLine("Hello" + request.Name);
            return Task.FromResult(new HelloReply { Message = "Hello" + request.Name });
        }
    }
    class Program
    {
        const int Port = 50051;
        public static void Main(string[] args)
        {
            string address = "0.0.0.0";
            Grpc.Core.Server server = new Grpc.Core.Server
            {
                Services = { Greeter.BindService(new TalkImpl()) },
                Ports = { new ServerPort(address, Port, ServerCredentials.Insecure) }
            };
            server.Start();
            Console.WriteLine("Greeter server listening on port " + Port);
            Console.WriteLine("Press any key to stop the server...");
            Console.ReadKey();

            server.ShutdownAsync().Wait();
        }
    }
}
