import { View, StyleSheet, Text, VirtualizedList } from "react-native";


export default function Home(options) {


    return (
        <View style={styles.container}>
            <View style={styles.Drawer}>
                <Text>This is the App Bar</Text>
            </View>
            
            <View style={styles.mainContainer}>
                <View style={styles.topBar}>
                    <Text>This is the Home screen!</Text>
                </View>

                <View style={[{flex: 0.1}]}></View>

                <View style={styles.buglistContainer}>
                    <Text>This is the list section</Text>
                    {/* <VirtualizedList /> */}
                </View>
            </View>
           
        </View>
    )

}

const styles = StyleSheet.create({
    container: {
        backgroundColor: 'rgb(255, 255, 255)',
        alignItems: 'center',
        justifyContent: 'center',
        width: '100%',
        height: '100%',
        position: 'absolute',
        flexDirection: 'row',
    },
    mainContainer: {
        flex: 10,
        height: '100%',
        flexDirection: 'column',
        justifyContent: 'space-between',
    },
    Drawer: {
        backgroundColor: 'rgb(50, 150, 255)',
        borderBottomRightRadius: '1rem',
        borderTopRightRadius: '1rem',
        flex: 2,
        height: "100%",
        top: '0',
        left: '0',
        alignItems: 'center',
        justifyContent: 'center',
    }, 
    topBar: {
        top: '0',
        flex: 1,
        width: '90%',
        backgroundColor: 'rgba(255, 0, 0, 0.5)',
        alignSelf: 'center',
        borderRadius: '2rem',
        top: '0.25rem',
        justifyContent: 'center',
        alignItems: 'center',
    },
    buglistContainer: {
        backgroundColor: 'rgba(0, 0, 0, 0.5)',
        flex: 5,
        width: '90%',
        paddingVertical: '-1rem',
        borderRadius: '2rem',
        alignSelf: 'center',
        alignItems: 'center',
        justifyContent: 'center',
        bottom: '0.25rem',
    }
})