using System;
using System.Collections.Generic;
using System.Text;

namespace PasswordBasedHashing
{
    public interface IHashEncryption
    {
        string Hash(string input);
        (bool Verified, bool NeedsUpgrade) Check(string hash, string input);
    }
}
