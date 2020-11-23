using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using Microsoft.Extensions.Options;

namespace PasswordBasedHashing
{
    public class HashEncryptionService : IHashEncryption
    {

        private const int SaltSize = 16; // 128 bit
        private const int KeySize = 32; //256 bit

        /// <summary>
        /// <para>Initializer for the service.</para>
        /// </summary>
        /// <param name="options">How many iterations for PBKDF2</param>
        public HashEncryptionService(IOptions<HashingOptions> options)
        {
            Options = options.Value;
        }

        private HashingOptions Options { get; }

        /// <summary>
        /// <para>Given an input string, this method returns an encrypted hash in the format of `{iterations}.{salt}.{hash}`.</para>
        /// <para>Uses the SHA256 hash algorithm and [System.Security.Cryptography.Rfc2898DeriveBytes]'s PBKDF2.</para>
        /// <para>Salt size and key size are hard coded. Iteration number is passed into service initialization options.</para>
        /// </summary>
        /// <param name="input">The input string to be hashed.</param>
        /// <returns>Encrypted hash of the input string</returns>
        public string Hash(string input)
        {
            using Rfc2898DeriveBytes algorithm = new Rfc2898DeriveBytes(
                input,
                SaltSize,
                Options.Iterations,
                HashAlgorithmName.SHA256);
            string key = Convert.ToBase64String(algorithm.GetBytes(KeySize));
            string salt = Convert.ToBase64String(algorithm.Salt);

            return $"{Options.Iterations}.{salt}.{key}";
        }

        /// <summary>
        /// <para>Determines equality for provided hash and input strings.</para>
        /// <para>Hashes the input string, then compares it's bytes to the provided hash's bytes for equality.</para>
        /// </summary>
        /// <param name="hash">A hashed value to compare with the input string.</param>
        /// <param name="input">Assumed original string value of the provided hash.</param>
        /// <returns>A bool indicating equality, and another bool indicating whether the provided hash's iteration count
        /// does not equal the current service's iteration count.</returns>
        public (bool Verified, bool NeedsUpgrade) Check(string hash, string input)
        {
            string[] parts = hash.Split('.', 3);

            if (parts.Length != 3)
            {
                throw new FormatException("Unexpected hash format. " +
                                          "Should be formatted as `{iterations}.{salt}.{hash}`");
            }

            int iterations = Convert.ToInt32(parts[0]);
            byte[] salt = Convert.FromBase64String(parts[1]);
            byte[] key = Convert.FromBase64String(parts[2]);

            bool needsUpgrade = iterations != Options.Iterations;

            using Rfc2898DeriveBytes algorithm = new Rfc2898DeriveBytes(
                input,
                salt,
                iterations,
                HashAlgorithmName.SHA256);
            byte[] keyToCheck = algorithm.GetBytes(KeySize);

            bool verified = keyToCheck.SequenceEqual(key);

            return (verified, needsUpgrade);
        }
    }
}
