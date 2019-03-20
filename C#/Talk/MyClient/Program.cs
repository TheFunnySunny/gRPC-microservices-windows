using System;
using Grpc.Core;
using Talk;
namespace MyClient
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Enter your name: ");
            String user = Console.ReadLine();
            Channel channel = new Channel("127.0.0.1:50051", ChannelCredentials.Insecure);
            var client = new Greeter.GreeterClient(channel);
            HelloReply reply = client.SayHello(new HelloRequest { Name = user });
            Console.WriteLine(reply.Message);
            Console.WriteLine("Press any key to exit...");
            Console.ReadKey();
        }
    }
}
