using System;
using Microsoft.Extensions.Options;
using PasswordBasedHashing;

namespace ConsolePBH
{
    public class Program
    {

        private static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");

            IOptions<HashingOptions> hashingOptions = new OptionsWrapper<HashingOptions>(new HashingOptions(10000));
            HashEncryptionService hashEncryptionService = new HashEncryptionService(hashingOptions);
        }

    }
}
