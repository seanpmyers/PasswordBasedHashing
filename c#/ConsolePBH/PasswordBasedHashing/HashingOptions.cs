using System;
using System.Collections.Generic;
using System.Text;

namespace PasswordBasedHashing
{
    public sealed class HashingOptions
    {
        public HashingOptions()
        {

        }

        public HashingOptions(int iterations)
        {
            Iterations = iterations;
        }

        public int Iterations { get; set; } = 10000;
    }
}
