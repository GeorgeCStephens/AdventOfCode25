package uk.co.georgestephens.util;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Scanner;

public class InputReader {
	
	//TODO 1 Make get file as single string
	public static String getFileAsString(String filePath) throws IOException {
		System.out.println("Getting file: " + filePath);
		BufferedReader br = new BufferedReader(new FileReader(filePath));
		StringBuilder fileContents = new StringBuilder();
		try {
		    String line = br.readLine();

		    while (line != null) {
		        fileContents.append(line);
		        fileContents.append(System.lineSeparator());
		        line = br.readLine();
		    }
			return fileContents.toString();
		} finally {
		    br.close();
		}
	}
	
	//TODO 1 Make get file as ArrayList
	public static ArrayList<String> getFileAsArray(String filePath) {
		ArrayList<String> fileContents = new ArrayList<String>();
	
		// we create a scanner for reading the file
		try (Scanner scanner = new Scanner(Paths.get(filePath))) {

		    // we read the file until all lines have been read
		    while (scanner.hasNextLine()) {
		        // we read one line
		        String row = scanner.nextLine();
		        // we print the line that we read
		        fileContents.add(row);
		    }
		} catch (Exception e) {
		    System.out.println("Error: " + e.getMessage());
		}
		
		return fileContents;
	}

}
