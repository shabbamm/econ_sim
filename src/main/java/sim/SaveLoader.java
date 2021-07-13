package sim;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class SaveLoader {

    // This was in the GameState, but i felt it more appropriate to be in a SaveLoader/SaveHandler class of its own
    public static String loadConfig(String filename) throws FileNotFoundException {
        File file = new File(filename);
        Scanner scanner = new Scanner(file);
        String data = new String();

        while (scanner.hasNextLine()) {
            data += scanner.nextLine();
        }

        scanner.close();
        return data;
    }
}
